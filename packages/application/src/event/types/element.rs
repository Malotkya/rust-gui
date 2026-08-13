use super::{
    external::*,
    super::Event
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct MouseOverData {
    pub device_id: DeviceId,
    pub position: PhysicalPosition<f64>
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub struct KeyboardData {
    pub keyboard_id: DeviceId,
    pub physical_key: PhysicalKey,
    pub logical_key: Key,
    pub text: Option<SmolStr>,
    pub state: ElementState,
    pub repeat: bool,
}

impl KeyboardData {
    pub(crate) fn from(keyboard_id: DeviceId, event: KeyEvent) -> Self {
        let KeyEvent { physical_key, logical_key, text, state, repeat, .. }
            = event;
        Self {
            keyboard_id, physical_key,
            logical_key, text,
            state, repeat
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct MouseButtonData {
    pub button: MouseButton,
    pub position: PhysicalPosition<f64>,
    pub device_id: DeviceId
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct ScrollData {
    pub delta: MouseScrollDelta,
    pub phase: TouchPhase,
    pub device_id: DeviceId,
    pub position: Option<PhysicalPosition<f64>>
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct TouchpadPressure {
    pub device_id: DeviceId,
    pub pressure: f32,
    pub stage: i64
}

#[allow(non_upper_case_globals)]
impl Event {
    pub(crate) fn is_error(&self) -> bool {
        self.type_name.eq(Self::Error)
    }
    
    pub const MouseIn:&'static str = "mousein";
    pub const MouseOut:&'static str = "mouseout";
    pub const MouseOver:&'static str = "mouseover";
    pub const MouseDown:&'static str = "mousedown";
    pub const MouseUp:&'static str = "mouseup";
    pub const Input:&'static str = "input";
    pub const KeyDown:&'static str = "keydown";
    pub const KeyUp:&'static str = "keyup";
    pub const Scroll:&'static str = "scroll";
    pub const Error:&'static str = "error";
}