use std::any::Any;
use std::collections::HashMap;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::{
    atomic::{AtomicUsize, Ordering}
};
use std::fmt;

mod target;
pub use target::*;
mod types;
pub use types::*;
mod window;
pub(crate) use window::*;

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

    pub fn get_actionable(&self) -> Option<WinitEvent> {
        if !self.actionable {
            return None;
        }

        todo!()
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

