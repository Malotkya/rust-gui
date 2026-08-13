use std::{
    any::Any,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicUsize, Ordering},
    fmt
};

mod target;
pub use target::*;
mod types;
pub use types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BubbleEventState {
    Stop,
    Immediate,
    Continue
}

#[derive(Debug)]
pub struct Event {
    pub type_name: String,
    detail: Box<dyn Any>,
    bubbles: BubbleEventState,
    actionable: bool
}

#[derive(Debug)]
pub struct EventData<T: 'static> {
    pub type_name: String,
    pub detail: T,
    pub bubbles: BubbleEventState,
    pub actionable: bool
}

impl<T: 'static> Into<Event> for EventData<T> {
    fn into(self) -> Event {
        let EventData {type_name, detail, bubbles, actionable}
            = self;
        Event {
            type_name, bubbles, actionable,
            detail: Box::new(detail)
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let detail:&str = self.detail.downcast_ref::<String>()
            .map(|s|s.as_str())
            .unwrap_or("unknown");

        write!(f, "{}: {}", self.type_name, detail)
    }
}

impl Event {
    pub fn new<T:Any>(type_name:impl ToString, detail:T) -> Self {
        Self {
            type_name: type_name.to_string(),
            detail: Box::new(detail),
            bubbles: BubbleEventState::Continue,
            actionable: true
        }
    }

    pub(crate) fn get_actionable<T:TryFrom<Event>>(self) -> Result<Option<T>, T::Error> {
        if !self.actionable {
            return Ok(None);
        }

        TryInto::<T>::try_into(self)
            .map(|e|Some(e))
    }

    #[inline]
    pub fn is_actionable(&self) -> bool {
        self.actionable
    }

    #[inline]
    pub fn detail<T: 'static>(&self) -> Option<&T> {
        self.detail.downcast_ref()
    }

    #[inline]
    pub fn detail_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.detail.downcast_mut()
    }

    #[inline]
    pub fn deconstruct<T: 'static>(self) -> Result<EventData<T>, Event> {
        let Event { type_name, detail, bubbles, actionable } = self;
        match detail.downcast::<T>() {
            Ok(value) => Ok(EventData {
                type_name, bubbles, actionable,
                detail: *value
            }),
            Err(any) => Err(Event {
                type_name, bubbles, actionable,
                detail: any
            })
        }
    }

    #[inline]
    pub fn clone_detail<T: Clone + 'static>(&self) -> Option<T> {
        self.detail::<T>().map(|v|v.clone())
    }
}

impl UnwindSafe for Event {}
impl RefUnwindSafe for Event {}

pub type EventListener = fn(&Event) -> ();

#[derive(Debug)]
struct EventHandler {
    id: usize,
    listener: EventListener,
    once: bool
}

static NEXT_ID:AtomicUsize = AtomicUsize::new(0);

impl EventHandler {
    pub fn new(listener:EventListener, once:bool) -> Self {
        Self {
            listener, once,
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed)
        }
    }

    pub fn call(&self, event:&Event) -> Result<bool, String> {
        match std::panic::catch_unwind(||(self.listener)(event)) {
            Ok(_) => Ok(self.once),
            Err(e) => Err(
                e.downcast_ref::<String>().map(|s|s.clone())
                    .unwrap_or(format!("An unknown panic occured in event listneer {}!", self.id))
            )
        }
    }
}

