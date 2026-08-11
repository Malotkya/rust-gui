pub use winit::{
    dpi::PhysicalPosition,
    event::{
        WindowEvent as WinitEvent,
        AxisId, DeviceId, TouchPhase, MouseButton, MouseScrollDelta,
        ElementState, KeyEvent,
    },
    keyboard::{PhysicalKey, Key, SmolStr},
    window::Theme
};

#[derive(Debug, Clone, Copy)]
pub struct MouseButtonEventData {
    pub button: MouseButton,
    pub pos: PhysicalPosition<f64>,
    pub mouse_id: DeviceId
}

#[derive(Debug, Clone, Copy)]
pub struct MouseScrollEventData {
    pub delta: MouseScrollDelta,
    pub phase: TouchPhase,
    pub mouse_id: DeviceId
}

#[derive(Debug, Clone, Copy)]
pub struct MouseOverEventData {
    pub mouse_id: DeviceId,
    pub pos: PhysicalPosition<f64>
}

#[derive(Debug, Clone)]
pub struct KeyboardEventData {
    pub keyboard_id: DeviceId,
    pub physical_key: PhysicalKey,
    pub logical_key: Key,
    pub text: Option<SmolStr>,
    pub state: ElementState,
    pub repeat: bool,
}

impl KeyboardEventData {
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

#[derive(Debug, Clone, Copy)]
pub struct AxisEventData {
    pub device_id: DeviceId,
    pub axis: AxisId,
    pub value: f64
}

#[derive(Debug, Clone, Copy)]
pub struct TouchpadPressureEventData {
    pub device_id: DeviceId,
    pub pressure: f32,
    pub stage: i64
}

#[derive(Debug, Clone, Copy)]
pub struct GestureEventData<T: Copy + Clone> {
    pub delta: T,
    pub device_id:DeviceId,
    pub phase: TouchPhase
}