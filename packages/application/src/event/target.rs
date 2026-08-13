use super::*;
use std::{
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
    fmt::Display
};

#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct EventTargetInner {
    map: HashMap<String, Vec<EventHandler>>,
    parrent: Option<EventTargetCore>
}

pub(crate) type EventTargetCore = Rc<RefCell<EventTargetInner>>;

impl EventTargetInner {
    pub fn new() -> EventTargetCore {
        Rc::new(RefCell::new(
            Self {
                map: HashMap::new(),
                parrent: None
            }
        ))
    }

    pub fn new_parrent(parrent:EventTargetCore) -> EventTargetCore {
        Rc::new(RefCell::new(
            Self {
                map: HashMap::new(),
                parrent: Some(parrent)
            }
        ))
    }

    fn handle_event(&mut self, event:&mut Event) -> Result<(), String> {
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
            self.map.insert(name, new_list);
        }

        response
    }

    fn dispatch_event(&mut self, event:&mut Event, error_count:usize) -> Result<(), String> {
        if error_count > 10 {
            return Err(format!("Recursion limit Hit when handleing '{}' Event!", event.type_name));
        }

        if let Err(msg) = self.handle_event(event) {
            *event = Event::new("Error", msg);
            return self.dispatch_event(event, error_count+1);
        }

        if event.bubbles == BubbleEventState::Continue {
            if let Some(parrent) = &self.parrent {
                let mut parrent_inner = parrent.borrow_mut();
                return parrent_inner.dispatch_event(event, error_count);
            }
        } else if event.type_name.eq("error") {
            if let Some(parrent) = &self.parrent {
                let mut parrent_inner = parrent.borrow_mut();
                return parrent_inner.dispatch_event(event, error_count);
            }

            let message = event.detail.downcast_ref::<String>()
                .map(|s|s.as_str())
                .unwrap_or("Unknown Error Event Message");

            return Err(message.to_string())
        }

        Ok(())
    }

    fn add_handler(&mut self, type_name: &str, handler:EventHandler) -> usize {
        let id = handler.id;
        self.map.entry(type_name.to_ascii_lowercase())
            .or_insert(Vec::with_capacity(1))
            .push(handler);
        id
    }

    fn remove_handler(&mut self, id:usize) -> Option<EventHandler> {
        for (_, list) in &mut self.map {
            let result = list.extract_if(0..=list.len(), |handler|handler.id == id).next();
            if result.is_some() {
                return result;
            }
        }

        None
    }
}

pub(crate) trait ParentEventTarget {
    fn inner_ref(&self) -> EventTargetCore;
}

impl<T:ParentEventTarget> ParentEventTarget for &T {
    fn inner_ref(&self) -> EventTargetCore {
        (**self).inner_ref()
    }
}

impl<T:ParentEventTarget> ParentEventTarget for &mut T {
    fn inner_ref(&self) -> EventTargetCore {
        (**self).inner_ref()
    }
}

pub trait EventTarget {
    fn dispatch_event(&self, event:&mut Event) -> Result<(), impl Display>;
    fn add_event_listener(&mut self, type_name:&str, listener:EventListener) -> usize;
    fn add_event_listener_once(&mut self, type_name:&str, listener:EventListener) -> usize;
    fn remove_event_listener(&mut self, id:usize) -> Option<EventListener>;
}

impl<T:EventTarget> EventTarget for &mut T {
    fn dispatch_event(&self, event:&mut Event) -> Result<(), impl Display> {
        (**self).dispatch_event(event)
    }

    fn add_event_listener(&mut self, type_name:&str, listener:EventListener) -> usize {
        (**self).add_event_listener(type_name, listener)
    }

    fn add_event_listener_once(&mut self, type_name:&str, listener:EventListener) -> usize {
        (**self).add_event_listener_once(type_name, listener)
    }

    fn remove_event_listener(&mut self, id:usize) -> Option<EventListener> {
        (**self).remove_event_listener(id)
    }
}

impl EventTarget for EventTargetCore {
    fn add_event_listener(&mut self, type_name:&str, listener:EventListener) -> usize {
        self.borrow_mut().add_handler(
            type_name,
            EventHandler::new(listener, false)
        )
    }

    fn add_event_listener_once(&mut self, type_name:&str, listener:EventListener) -> usize {
        self.borrow_mut().add_handler(
            type_name, 
            EventHandler::new(listener, true)
        )
    }

    fn remove_event_listener(&mut self, id:usize) -> Option<EventListener> {
        self.borrow_mut().remove_handler(id)
            .map(|h|h.listener)
    }

    fn dispatch_event(&self, event:&mut Event) -> Result<(), impl Display> {
        self.borrow_mut()
            .dispatch_event(event, 0)
    }
}

impl ParentEventTarget for EventTargetCore {
    fn inner_ref(&self) -> EventTargetCore {
        self.clone()
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct WindowEventTarget{
    core: EventTargetCore,
    history: EventHistory
}

impl WindowEventTarget {
    pub fn new() -> Self {
        Self{
            core: EventTargetInner::new(),
            history: EventHistory::new()
        }
    }

    pub fn new_parrent(parrent:&impl ParentEventTarget) -> Self {
        Self{
            core: EventTargetInner::new_parrent(parrent.inner_ref()),
            history: EventHistory::new()
        }
    }

    pub(crate) fn handle_external_event(&mut self, event:impl ExternalEvent) -> EventResponse {
        EventResponse::from_extenral_event(event, &mut self.history)
    }
}

impl EventTarget for WindowEventTarget {
    fn dispatch_event(&self, event:&mut Event) -> Result<(), impl Display> {
        self.core.borrow_mut()
            .dispatch_event(event, 0)
    }

    fn add_event_listener(&mut self, type_name:&str, listener:EventListener) -> usize{
        self.core.borrow_mut()
            .add_handler(
                type_name,
                EventHandler::new(listener, false)
            )
    }

    fn add_event_listener_once(&mut self, type_name:&str, listener:EventListener) -> usize{
        self.core.borrow_mut()
            .add_handler(
                type_name,
                EventHandler::new(listener, true)
            )
    }

    fn remove_event_listener(&mut self, id:usize) -> Option<EventListener> {
        self.core.borrow_mut()
            .remove_handler(id)
            .map(|h|h.listener)
    }
}

impl ParentEventTarget for WindowEventTarget {
    fn inner_ref(&self) -> EventTargetCore {
        self.core.clone()
    }
}