use std::any::Any;
use std::collections::HashMap;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::rc::{Weak, Rc};
use std::sync::{
    atomic::{AtomicUsize, Ordering}
};
use std::fmt;

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

    pub fn is_actionable(&self) -> bool {
        self.actionable
    }
}

impl UnwindSafe for Event {}
impl RefUnwindSafe for Event {}

pub type EventListener = fn(&Event) -> ();

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

pub struct EventTarget {
    map: HashMap<String, Vec<EventHandler>>,
    pub(crate) parrent:Option<Weak<EventTarget>>,
}

impl EventTarget {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            map: HashMap::new(),
            parrent: None
        })
    }

    pub fn new_parrent(parrent:&Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            map: HashMap::new(),
            parrent: Some(Rc::downgrade(parrent))
        })
    }

    #[inline]
    #[allow(invalid_reference_casting)]
    fn deref_mut(self: &Rc<Self>) -> &mut Self {
        unsafe {
            &mut *(
                (self as *const Rc<Self> as *mut Rc<Self> as *mut u8) as *mut Self
            )
        }
    }

    pub(crate) fn parrent_mut(&mut self) -> Option<&mut Self> {
        if let Some(parrent) = &mut self.parrent{
            let ptr = parrent as *mut Weak<Self> as *mut u8;
            Some( unsafe {
                 &mut *(ptr as *mut Self)
            } );
        }

        None
    }

    pub fn parrent(&self) -> Option<&Self> {
        if let Some(parrent) = &self.parrent {
            let ptr = parrent as *const Weak<EventTarget> as *const u8;
            Some( unsafe {
                & *(ptr as *const EventTarget)
            } );
        }

        None
    }

    fn handle_event(&mut self, event:&Event) -> Result<(), String> {
        let name = event.type_name.to_ascii_lowercase();
        let mut response:Result<(), String> = Ok(());

        if let Some(list) = self.map.remove(&name) {

            let mut new_list = Vec::with_capacity(list.len());
            let mut it = list.into_iter();

            while let Some(handler) = it.next() {
                match handler.call(event) {
                    Ok(once) => if !once {
                        new_list.push(handler)
                    },
                    Err(msg) => {
                        response = Err(msg);
                        break;
                    }
                }

                if event.bubbles == BubbleEventState::Stop {
                    break;
                }
            }

            new_list.extend(it);
        }

        response
    }

    fn dispatch_inner(&mut self, event:&Event, error_count:usize) -> Result<(), &'static str> {
        if error_count > 10 {
            return Err("Possible recursion occured!");
        }

        if let Err(msg) = self.handle_event(event) {
            return self.dispatch_inner(&Event::new("Error", msg), error_count+1);
        }

        if event.bubbles == BubbleEventState::Continue && let Some(parrent) = self.parrent_mut() {
            return parrent.dispatch_inner(event, error_count);
        }

        Ok(())
    }

    pub fn dispatch_event(self:&Rc<Self>, event: &Event) -> Result<(), &'static str>{
        self.deref_mut().dispatch_inner(event, 0)
    }

    fn add_handler(&mut self, type_name: &str, handler:EventHandler) -> usize {
        let id = handler.id;
        self.map.entry(type_name.to_ascii_lowercase())
            .or_insert(Vec::with_capacity(1))
            .push(handler);
        id
    }

    pub fn add_event_listener(self:&Rc<Self>, type_name: &str, listener: EventListener) -> usize {
        self.deref_mut().add_handler(
            type_name,
            EventHandler::new(listener, false)
        )
    }

    pub fn add_event_listener_once(self:&Rc<Self>, type_name: &str, listener: EventListener) -> usize {
        self.deref_mut().add_handler(
            type_name,
            EventHandler::new(listener, true)
        )
    }
}