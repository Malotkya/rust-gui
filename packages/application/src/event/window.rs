use std::time::Instant;
use super::*;

#[derive(Debug)]
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

pub(crate) enum ApplicationEvent {
    ElementEvent(Event),
    WindowEvent(Event),
    WinitEvent(WinitEvent),
    None
}

impl ApplicationEvent {
    pub fn from_winit_event(event:WinitEvent, mouse:&mut EventHistory, touch:&mut EventHistory) -> Self {
        match event {
            WinitEvent::Resized(size) => Self::ElementEvent(
                Event::new("resized", size)
            ),
            WinitEvent::Focused(focus) => if focus {
                Self::ElementEvent(Event::new("focus", ()))
            } else {
                Self::ElementEvent(Event::new("blur", ()))
            },
            WinitEvent::Ime(_ime) => todo!("Ime to input event"),
            WinitEvent::MouseInput{device_id, state, button} => if let Some(pos) = mouse.get(&device_id, None) {
                match state {
                    ElementState::Pressed => Self::ElementEvent(Event::new("mousedown", MouseButtonEventData{
                        button, pos,
                        mouse_id: device_id,
                    })),
                    ElementState::Released => Self::ElementEvent(Event::new("mouseup", MouseButtonEventData {
                        button, pos,
                        mouse_id: device_id
                    }))
                }
            } else {
                Self::None
            },
            WinitEvent::CursorMoved { device_id, position } => {
                mouse.update(&device_id, position);
                Self::ElementEvent(Event::new("mouseover", MouseOverEventData{
                    pos: position,
                    mouse_id: device_id
                }))
            },
            WinitEvent::CursorLeft { device_id } => {
                mouse.remove(&device_id);
                Self::ElementEvent(Event::new("mouseleave", device_id))
            },
            WinitEvent::CursorEntered { device_id } => Self::ElementEvent(
                Event::new("mouseenter", device_id)
            ),
            WinitEvent::MouseWheel { device_id, delta, phase } => Self::ElementEvent(
                Event::new("scroll", MouseScrollEventData{
                    delta, phase,
                    mouse_id: device_id
                })
            ),
            WinitEvent::Touch(t) =>{
                touch.update(&t.device_id, t.location);
                Self::ElementEvent(Event::new("touch", t))
            },
            WinitEvent::KeyboardInput { device_id, event, is_synthetic: _ } => match event.state {
                ElementState::Pressed => Self::ElementEvent(
                    Event::new("keydown", KeyboardEventData::from(device_id, event))
                ),
                ElementState::Released => Self::ElementEvent(
                    Event::new("keyup", KeyboardEventData::from(device_id, event))
                )
            },
            WinitEvent::DoubleTapGesture { device_id } => if let Some(pos) = touch.get(&device_id, Some(300)) {
                Self::ElementEvent(Event::new("doubletap", pos))
            } else {
                Self::None
            },
            WinitEvent::Moved(pos) => Self::WindowEvent(
                Event::new("moved", pos)
            ),
            WinitEvent::CloseRequested => Self::WindowEvent(
                Event::new("closed", ())
            ),
            WinitEvent::HoveredFile(path) => Self::WindowEvent(
                Event::new("drag", path)
            ),
            WinitEvent::DroppedFile(path) => Self::WindowEvent(
                Event::new("drop", path)
            ),
            WinitEvent::HoveredFileCancelled => Self::WindowEvent(
                Event::new("dragcanceled", ())
            ),
            WinitEvent::TouchpadPressure { device_id, pressure, stage } => Self::WindowEvent(
                Event::new("touchpadpressure", TouchpadPressureEventData{
                    pressure, stage, device_id
                })
            ),
            WinitEvent::PinchGesture { device_id, delta, phase } => Self::WindowEvent(
                Event::new("pinch", GestureEventData{
                    device_id, delta, phase
                })
            ),
            WinitEvent::PanGesture { device_id, delta, phase } => Self::WindowEvent(
                Event::new("pan", GestureEventData{
                    device_id, delta, phase
                })
            ),
            WinitEvent::RotationGesture { device_id, delta, phase } => Self::WindowEvent(
                Event::new("rotation", GestureEventData{
                    device_id, delta, phase
                })
            ),
            WinitEvent::AxisMotion { device_id, axis, value } => Self::WindowEvent(
                Event::new("axismotion", AxisEventData{
                    axis, value, device_id
                })
            ),
            WinitEvent::ThemeChanged(theme) => Self::WindowEvent(
                Event::new("themechanged", theme)
            ),
            WinitEvent::Occluded(focus) => Self::WindowEvent(
                Event::new(if focus {"maximized"} else {"minimized"}, ())
            ),
            event => Self::WinitEvent(event)
        }
    }
}
