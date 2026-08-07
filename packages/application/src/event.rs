use winit::{
    dpi::{PhysicalPosition, PhysicalSize}, event::*,
    event_loop::{ActiveEventLoop},
    keyboard::{Key, PhysicalKey, SmolStr},
    window::Theme
};
pub use winit::event::WindowEvent as WinitEvent;
use std::{
    any::Any,
    rc::Rc,
    path::PathBuf,
    time::Instant,
    cell::RefCell,
    collections::LinkedList,
};

#[derive(Debug)]
pub struct Event<'a, T> {
    event_loop: Option<&'a ActiveEventLoop>,
    pub detail: &'a T
}

impl<'a, T> Event<'a, T> {
    fn new(event_loop: &'a ActiveEventLoop, detail:&'a T) -> Self {
        Self {
            event_loop: Some(event_loop), detail
        }
    }
}

impl<'a> Event<'a, ()> {
    fn blank(event_loop: &'a ActiveEventLoop) -> Self {
        Self {
            event_loop: Some(event_loop),
            detail: &()
        }
    }

    fn empty() -> Self {
        Self {
            event_loop: None,
            detail: &()
        }
    }
}

impl<'a, T> From<&'a T> for  Event<'a, T> {
    fn from(value:&'a T) -> Self {
        Self{
            event_loop: None,
            detail: value
        }
    }
}

pub type EventListener<T> = fn(Event<'_, T>) -> ();

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
    fn from(id: DeviceId, event: &KeyEvent) -> Self {
        Self {
            keyboard_id: id,
            physical_key: event.physical_key,
            logical_key: event.logical_key.clone(),
            text: event.text.clone(),
            state: event.state,
            repeat: event.repeat
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GestureEventType {
    Pinch(f64),
    Pan(PhysicalPosition<f32>),
    Rotation(f32)
}

#[derive(Debug, Clone, Copy)]
pub struct GestureEventData {
    gesture: GestureEventType,
    device_id:DeviceId,
    phase: TouchPhase
}

pub enum ContextEvent {
    Resized(PhysicalSize<u32>),
    Focus,
    Blur,
    Input,
    MouseDown(MouseButtonEventData),
    MouseUp(MouseButtonEventData),
    MouseOver(MouseOverEventData),
    MouseEnter(DeviceId),
    MouseLeave(DeviceId),
    Scroll(MouseScrollEventData),
    Touch(Touch),
    KeyDown(KeyboardEventData),
    KeyUp(KeyboardEventData),
    Gesture(GestureEventData),
    DoubleTap(PhysicalPosition<f64>),
    Custom(String, Rc<dyn Any>),
    Error(String)
}

impl Into<WindowEvent> for ContextEvent {
    fn into(self) -> WindowEvent {
        WindowEvent::Context(self)
    }
}

impl ContextEvent {
    pub fn custom_event<S:ToString>(name:S, detail: impl Any) -> Self {
        Self::Custom(name.to_string().to_lowercase(), Rc::new(detail))
    }
}

pub enum ContextEventListener {
    Resized(EventListener<PhysicalSize<u32>>),
    Focus(EventListener<()>),
    Blur(EventListener<()>),
    Input(EventListener<()>),
    MouseDown(EventListener<MouseButtonEventData>),
    MouseUp(EventListener<MouseButtonEventData>),
    MouseOver(EventListener<MouseOverEventData>),
    MouseEnter(EventListener<DeviceId>),
    MouseLeave(EventListener<DeviceId>),
    Scroll(EventListener<MouseScrollEventData>),
    Touch(EventListener<Touch>),
    KeyDown(EventListener<KeyboardEventData>),
    KeyUp(EventListener<KeyboardEventData>),
    Gesture(EventListener<GestureEventData>),
    DoubleTap(EventListener<PhysicalPosition<f64>>),
    Custom(String, EventListener<Rc<dyn Any>>),   
    Error(EventListener<String>)
}

impl Into<WindowEventListener> for ContextEventListener {
    fn into(self) -> WindowEventListener {
        WindowEventListener::Context(self)
    }
}

pub(crate) enum MouseHistory {
    Some(DeviceId, PhysicalPosition<f64>, Instant),
    None
}

impl MouseHistory {
    pub(crate) fn new(id:DeviceId, pos:PhysicalPosition<f64>) -> Self {
        Self::Some(id, pos, Instant::now())
    }

    pub(crate) fn get(&self, device_id:&DeviceId, timeout:Option<u64>) -> Option<PhysicalPosition<f64>> {
        if let MouseHistory::Some(id, pos, inst) = self {
            if let Some(ttl) = timeout && inst.elapsed().as_micros() > ttl.into() {
                return None;
            }

            if *device_id == *id {
                return Some(*pos)
            }

        }

        None
    }
}

impl ContextEventListener {
    pub(crate) fn match_call(&self, event: &WinitEvent, event_loop:&ActiveEventLoop, mouse_pos:&mut MouseHistory, touch_pos:&mut MouseHistory) -> bool {
        match event {
            WinitEvent::Resized(detail) => match self {
                Self::Resized(func) => {
                    func(Event::new(event_loop, detail));
                    true
                },
                _ => false
            }
            WinitEvent::Focused(focus) => match self {
                Self::Focus(func) => if *focus {
                    func(Event::blank(event_loop));
                    true
                } else {
                    false
                },
                Self::Blur(func) => if !(*focus) {
                    func(Event::blank(event_loop));
                    true
                } else {
                    false
                },
                _ => false
            }
            WinitEvent::Ime(_ime) => match self {
                Self::Input(_func) => {
                    todo!("Figure out ime and input correlation!")
                },
                _ => false
            },
            WinitEvent::MouseInput { device_id, state, button } => if let Some(pos) = mouse_pos.get(device_id, None) {
                match self {
                    Self::MouseDown(func) => if *state == ElementState::Pressed {
                        let detail = MouseButtonEventData{pos, button: *button, mouse_id: *device_id};
                        func(Event::new(event_loop, &detail));
                        true
                    } else {
                        false
                    },
                    Self::MouseUp(func) => if *state == ElementState::Released {
                        let detail = MouseButtonEventData{pos, button: *button, mouse_id: *device_id};
                        func(Event::new(event_loop, &detail));
                        true
                    } else {
                        false
                    },
                    _ => false
                } 
            } else {
                false
            },
            WinitEvent::CursorMoved { device_id, position } => {
                *mouse_pos = MouseHistory::new(*device_id, *position);
                match self {
                    Self::MouseOver(func) => {
                        let detail = MouseOverEventData{pos: *position, mouse_id: *device_id};
                        func(Event::new(event_loop, &detail));
                        true
                    },
                    _ => false
                }
            },
            WinitEvent::CursorLeft { device_id } => {
                *mouse_pos = MouseHistory::None;
                match self {
                    Self::MouseLeave(func) => {
                        func(Event::new(event_loop, device_id));
                        true
                    },
                    _ => false
                }
            },
            WinitEvent::CursorEntered { device_id } => match self {
                Self::MouseEnter(func) => {
                    func(Event::new(event_loop, device_id));
                    true
                },
                _ => false
            }
            WinitEvent::MouseWheel { device_id, delta, phase } => match self {
                Self::Scroll(func) => {
                    let detail = MouseScrollEventData{delta: *delta, phase: *phase, mouse_id: *device_id};
                    func(Event::new(event_loop, &detail));
                    true
                },
                _ => false
            },
            WinitEvent::Touch(t) => {
                *touch_pos = MouseHistory::new(t.device_id, t.location);
                match self {
                    Self::Touch(func) => {
                        func(Event::new(event_loop, t));
                        true
                    },
                    _ => false
                }
            },
            WinitEvent::KeyboardInput { event, device_id, is_synthetic: _ } => match self {
                Self::KeyDown(func) => if event.state == ElementState::Pressed {
                    let detail = KeyboardEventData::from(*device_id, event);
                    func(Event::new(event_loop, &detail));
                    true
                } else {
                    false
                },
                Self::KeyUp(func) => if event.state == ElementState::Released {
                    let detail = KeyboardEventData::from(*device_id, event);
                    func(Event::new(event_loop, &detail));
                    true
                } else {
                    false
                },
                _ => false
            },
            WinitEvent::PinchGesture { device_id, delta, phase } => match self {
                Self::Gesture(func) => {
                    let detail = GestureEventData{
                        gesture: GestureEventType::Pinch(*delta),
                        device_id: *device_id,
                        phase: *phase
                    };
                    func(Event::new(event_loop, &detail));
                    true
                },
                _ => false
            },
            WinitEvent::PanGesture { device_id, delta, phase } => match self {
                Self::Gesture(func) => {
                    let detail = GestureEventData{
                        gesture: GestureEventType::Pan(*delta),
                        device_id: *device_id,
                        phase: *phase
                    };
                    func(Event::new(event_loop, &detail));
                    true
                },
                _ => false
            },
            WinitEvent::RotationGesture { device_id, delta, phase } => match self {
                Self::Gesture(func) => {
                    let detail = GestureEventData{
                        gesture: GestureEventType::Rotation(*delta),
                        device_id: *device_id,
                        phase: *phase
                    };
                    func(Event::new(event_loop, &detail));
                    true
                },
                _ => false
            },
            WinitEvent::DoubleTapGesture { device_id } => match self {
                Self::DoubleTap(func) => if let Some(pos) = mouse_pos.get(device_id, Some(300))
                    .or(touch_pos.get(device_id, Some(300)))
                {
                    func(Event::new(event_loop, &pos));
                    true
                } else {
                    false
                },
                _ => false
            },
            _ => false
        }
    }

    pub(crate) fn event_call(&self, event:&ContextEvent, event_loop:Option<&ActiveEventLoop>) -> bool {
        match event {
            ContextEvent::Blur => match self {
                Self::Blur(func) => {
                    func(Event{event_loop, detail: &()});
                    true
                },
                _ => false
            },
            ContextEvent::Custom(name, detail) => match self {
                Self::Custom(self_name, func) => if name.to_lowercase().as_str() == self_name {
                    func(Event{event_loop, detail});
                    true
                } else {
                    false
                },
                _ => false
            },
            ContextEvent::DoubleTap(detail) => match self {
                Self::DoubleTap(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::Focus => match self {
                Self::Focus(func) => {
                    func(Event{event_loop, detail: &()});
                    true
                }
                _ => false
            },
            ContextEvent::Gesture(detail) => match self {
                Self::Gesture(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::Input => match self {
                Self::Input(func) => {
                    func(Event{event_loop, detail: &()});
                    true
                }
                _ => false
            },
            ContextEvent::KeyDown(detail) => match self {
                Self::KeyDown(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::KeyUp(detail) => match self {
                Self::KeyUp(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::MouseDown(detail) => match self {
                Self::MouseDown(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::MouseUp(detail) => match self {
                Self::MouseUp(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::MouseEnter(detail) => match self {
                Self::MouseEnter(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::MouseLeave(detail) => match self {
                Self::MouseLeave(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::MouseOver(detail) => match self {
                Self::MouseOver(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::Resized(detail) => match self {
                Self::Resized(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::Scroll(detail) => match self {
                Self::Scroll(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            },
            ContextEvent::Touch(detail) => match self {
                Self::Touch(func) => {
                    func(Event{event_loop, detail});
                    true
                }
                _ => false
            }
            ContextEvent::Error(detail) => match self {
                Self::Error(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AxisEventData {
    axis: AxisId,
    value: f64
}

#[derive(Debug, Clone, Copy)]
pub struct TouchpadPressureEventData {
    pressure: f32,
    stage: i64
}

pub enum WindowEvent {
    Context(ContextEvent),
    Moved(PhysicalPosition<i32>),
    Closed,
    DroppedFile(PathBuf),
    HoveredFile(PathBuf),
    HoveredFileCancelled,
    TouchpadPressure(TouchpadPressureEventData),
    AxisMotion(AxisEventData),
    Theme(Theme),
    Occluded(bool)
}

impl WindowEvent {
    pub fn custom_event<S:ToString>(name:S, detail: impl Any) -> Self {
        Self::Context(ContextEvent::custom_event(name, detail))
    }
}

pub enum WindowEventListener {
    Context(ContextEventListener),
    Moved(EventListener<PhysicalPosition<i32>>),
    Closed(EventListener<()>),
    DroppedFile(EventListener<PathBuf>),
    HoveredFile(EventListener<PathBuf>),
    HoveredFileCancelled(EventListener<()>),
    TouchpadPressure(EventListener<TouchpadPressureEventData>),
    AxisMotion(EventListener<AxisEventData>),
    ThemeChanged(EventListener<Theme>),
    Occluded(EventListener<bool>)
}

impl WindowEventListener {
    pub(crate) fn match_call(&self, event: &WinitEvent, event_loop:&ActiveEventLoop, mouse_pos:&mut MouseHistory, touch_pos:&mut MouseHistory) -> bool {
        match event {
            WinitEvent::Moved(pos) => match self {
                Self::Moved(func) => {
                    func(Event::new(event_loop, pos));
                    true
                },
                _ => false
            },
            WinitEvent::CloseRequested => match self {
                Self::Closed(func) => {
                    func(Event::blank(event_loop));
                    true
                },
                _ => false
            },
            WinitEvent::HoveredFile(path) => match self {
                Self::DroppedFile(func) => {
                    func(Event::new(event_loop, path));
                    true
                },
                _ => false
            },
            WinitEvent::DroppedFile(path) => match self {
                Self::DroppedFile(func) => {
                    func(Event::new(event_loop, path));
                    true
                },
                _ => false
            },
            WinitEvent::HoveredFileCancelled => match self {
                Self::HoveredFileCancelled(func) => {
                    func(Event::blank(event_loop));
                    true
                },
                _ => false
            },
            WinitEvent::TouchpadPressure{device_id: _, pressure, stage} => match self {
                Self::TouchpadPressure(func) => {
                    let detail = TouchpadPressureEventData { pressure: *pressure, stage: *stage };
                    func(Event::new(event_loop, &detail));
                    true
                },
                _ => false
            },
            WinitEvent::AxisMotion { device_id: _, axis, value } => match self {
                Self::AxisMotion(func) => {
                    let detail = AxisEventData { axis: *axis, value: *value };
                    func(Event::new(event_loop, &detail));
                    true
                },
                _ => false
            },
            WinitEvent::ThemeChanged(theme) => match self {
                Self::ThemeChanged(func) => {
                    func(Event::new(event_loop, theme));
                    true
                },
                _ => false
            },
            WinitEvent::Occluded(focus) => match self {
                Self::Occluded(func) => {
                    func(Event::new(event_loop, focus));
                    true
                },
                _ => false
            },
            _ => match self {
                Self::Context(listener) =>
                    listener.match_call(event, event_loop, mouse_pos, touch_pos),
                _ => false
            }
        }
    }

    pub(crate) fn event_call(&self, event:&WindowEvent, event_loop:Option<&ActiveEventLoop>) -> bool {
        match event {
            WindowEvent::AxisMotion(detail) => match self {
                Self::AxisMotion(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            },
            WindowEvent::Closed => match self {
                Self::Closed(func) => {
                    func(Event { event_loop, detail: &() });
                    true
                },
                _ => false
            },
            WindowEvent::Context(c) => match self {
                Self::Context(listener) => 
                    listener.event_call(c, event_loop),
                _ => false
            },
            WindowEvent::DroppedFile(detail) => match self {
                Self::DroppedFile(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            },
            WindowEvent::HoveredFile(detail) => match self {
                Self::HoveredFile(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            },
            WindowEvent::HoveredFileCancelled => match self {
                Self::HoveredFileCancelled(func) => {
                    func(Event{event_loop, detail: &()});
                    true
                },
                _ => false
            },
            WindowEvent::Moved(detail) => match self {
                Self::Moved(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            },
            WindowEvent::Occluded(detail) => match self {
                Self::Occluded(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            },
            WindowEvent::Theme(detail) => match self {
                Self::ThemeChanged(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            },
            WindowEvent::TouchpadPressure(detail) => match self {
                Self::TouchpadPressure(func) => {
                    func(Event{event_loop, detail});
                    true
                },
                _ => false
            }
        }

    }
}

#[derive(Debug)]
pub enum GlobalEvent {

}

#[derive(Debug)]
pub enum GlobalEventListener {

}

#[derive(Debug, Clone)]
pub struct GlobalEventTarget(Rc<RefCell<LinkedList<GlobalEventListener>>>);

impl GlobalEventTarget {
    pub fn new() -> Self {
        Self(Rc::new(
            RefCell::new(
                LinkedList::new()
            )
        ))
    }

    pub fn append_event_listener(&self, listener: GlobalEventListener) {
        let mut list = self.0.borrow_mut();
        list.push_back(listener);
    }

    pub fn dispatch_event(&self, event:GlobalEvent) -> u32{
        let list = self.0.borrow();

        let mut count: u32 = 0;
        for listener in &*list {
            if listener.handle_event(&event) {
                count += 1;
            }
        }

        count
    }
}