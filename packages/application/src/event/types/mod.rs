use std::{
    collections::HashMap,
    time::Instant
};
use super::Event;

mod application;
pub use application::*;
mod element;
pub use element::*;
mod external;
pub use external::*;
mod window;
pub use window::*;

#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct EventHistory(HashMap<DeviceId, (PhysicalPosition<f64>, Instant)>);

impl EventHistory {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, device_id:&DeviceId, timeout:Option<u64>) -> Option<PhysicalPosition<f64>> {
        if let Some((pos, inst)) = self.0.get(device_id) {
            if let Some(ttl) = timeout && inst.elapsed().as_micros() > ttl.into() {
                return None;
            }

            return Some(*pos)
        }

        None
    }

    pub fn update(&mut self, device_id:&DeviceId, pos:PhysicalPosition<f64>) {
        self.0.insert(*device_id, (pos, Instant::now()));
    }

    pub fn remove(&mut self, device_id:&DeviceId) {
        self.0.remove(device_id);
    } 
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) enum EventResponse {
    ElementEvent(Event),
    WindowEvent(Event),
    ApplicationEvent(Event),
    Error(Event)
}

impl EventResponse {
    pub fn from_extenral_event(event: impl ExternalEvent, history:&mut EventHistory) -> Self {
        let event:Event = event.into_event(history);

        if event.is_error() {
            Self::Error(event)
        } else if event.is_application_event() {
            Self::ApplicationEvent(event)
        } else if event.is_window_event() {
            Self::WindowEvent(event)
        } else {
            Self::ElementEvent(event)
        }
    }
}