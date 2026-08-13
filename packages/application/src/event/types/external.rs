#[allow(unused_imports)]
pub use winit::{
    event_loop::AsyncRequestSerial,
    window::ActivationToken,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        DeviceEvent, WindowEvent as WinitEvent, RawKeyEvent as RawKeyData,
        MouseScrollDelta, ButtonId, ElementState, DeviceId, AxisId,
        TouchPhase, Force, Modifiers, InnerSizeWriter, KeyEvent, MouseButton
    },
    keyboard::{PhysicalKey, Key, SmolStr}
};
use winit::event::Touch;
use super::{
    *,
    super::Event
};

const MOUSE_POS_ERROR:&'static str = "Failed to get position of the mouse on input!";
const TAP_POS_ERROR:&'static str = "Failed to get postion on double tap!";

pub(crate) trait ExternalEvent {
    fn into_event(self, history:&mut EventHistory) -> Event;
}

impl ExternalEvent for DeviceEvent {
    fn into_event(self, _:&mut EventHistory) -> Event {
        match self {
            Self::Added =>
                Event::new(Event::DeviceAdded, ()),
            Self::Button { button, state } => match state {
                ElementState::Pressed => Event::new(Event::ButtonDown, button),
                ElementState::Released => Event::new(Event::ButtonUp, button)
            },
            Self::Key(data) => 
                Event::new(Event::KeyPressed, data),
            Self::Motion { axis, value } =>
                Event::new(Event::DeviceMotion, DeviceMotionData{axis, value}),
            Self::MouseMotion { delta } =>
                Event::new(Event::PointerMove, PhysicalPosition{x: delta.0, y:delta.1}),
            Self::MouseWheel { delta } =>
                Event::new(Event::MouseWheel, delta),
            Self::Removed =>
                Event::new(Event::DeviceRemoved, ())
        }
    }
}

impl ExternalEvent for WinitEvent {
    fn into_event(self, history:&mut EventHistory) -> Event {
        match self {
            Self::ActivationTokenDone { serial, token } =>
                Event::new(Event::ActivationTokenDone, ActivationTokenData{serial, token}),
            Self::AxisMotion { device_id, axis, value } =>
                Event::new(Event::AxisMotion, AxisMotionData{device_id, axis, value}),
            Self::CloseRequested =>
                Event::new(Event::CloseRequested, ()),
            Self::CursorEntered { device_id } =>
                Event::new(Event::MouseIn, device_id),
            Self::CursorLeft { device_id } => {
                history.remove(&device_id);
                Event::new(Event::MouseOut, device_id)
            },
            Self::CursorMoved { device_id, position } => {
                history.update(&device_id, position);
                Event::new(Event::MouseOver, MouseOverData{device_id, position})
            },
            Self::Destroyed =>
                Event::new(Event::Destoryed, ()),
            Self::DoubleTapGesture { device_id } => if let Some(pos) = history.get(&device_id, Some(300)) {
                Event::new(Event::DoubleTap, TouchData{
                    device_id,
                    phase: TouchPhase::Ended,
                    location: pos,
                    force: None,
                })
            } else {
                Event::new(Event::Error, TAP_POS_ERROR.to_string())
            },
            Self::DroppedFile(path) =>
                Event::new(Event::Drop, path),
            Self::HoveredFile(path) =>
                Event::new(Event::Dragged, path),
            Self::HoveredFileCancelled => 
                Event::new(Event::DraggedEnd, ()),
            Self::Focused(focus) => if focus {
                    Event::new(Event::Focus, ())
                } else {
                    Event::new(Event::Blur, ())
                },
            Self::Ime(_ime) => todo!("IME -> input"),
            Self::KeyboardInput { device_id, event, is_synthetic: _ } => match event.state {
                ElementState::Pressed => Event::new(Event::KeyDown, KeyboardData::from(device_id, event)),
                ElementState::Released => Event::new(Event::KeyUp, KeyboardData::from(device_id, event))
            },
            Self::ModifiersChanged(data) => 
                Event::new(Event::ModifiersChanged, data),
            Self::MouseInput { device_id, state, button } =>
                if let Some(pos) = history.get(&device_id, None){ match state {
                    ElementState::Pressed => Event::new(Event::MouseDown, MouseButtonData{
                        button, device_id,
                        position: pos
                    }),
                    ElementState::Released => Event::new(Event::MouseUp, MouseButtonData{
                        button, device_id,
                        position: PhysicalPosition { x: 0.0, y: 0.0 }
                    })
                } } else {
                    Event::new(Event::Error, MOUSE_POS_ERROR.to_string())
                },
            Self::MouseWheel { device_id, delta, phase } =>
                Event::new(Event::MouseWheel, ScrollData {
                    position: history.get(&device_id, None),
                    delta, phase, device_id
                }),
            Self::Moved(pos) =>
                Event::new(Event::Moved, pos),
            Self::Occluded(focus) => if focus {
                Event::new(Event::Maximized, ())
            } else {
                Event::new(Event::Minimized, ())
            },
            Self::PanGesture { device_id, delta, phase } =>
                Event::new(Event::PanGesture, GestureEventType::Pan(
                    GestureEventData{
                        device_id, delta, phase
                    }
                )),
            Self::PinchGesture { device_id, delta, phase } =>
                Event::new(Event::PanGesture, GestureEventType::Pinch(
                    GestureEventData{
                        device_id, delta, phase
                    }
                )),
            Self::RotationGesture { device_id, delta, phase } =>
                Event::new(Event::PanGesture, GestureEventType::Rotation(
                    GestureEventData{
                        device_id, delta, phase
                    }
                )),
            Self::RedrawRequested =>
                Event::new(Event::RedrawRequested, ()),
            Self::Resized(data) =>
                Event::new(Event::Resized, data),
            Self::ScaleFactorChanged { scale_factor, inner_size_writer } =>
                Event::new(Event::ScaleFactorChanged, ScaleFactorChanged{
                    scale_factor, inner_size_writer
                }),
            Self::ThemeChanged(data) =>
                Event::new(Event::ThemeChanged, data),
            Self::Touch(t) => {
                let Touch{device_id, phase, location, force, ..} = t;
                history.update(&device_id, location);
                Event::new(Event::Touch, TouchData {
                    device_id, phase, location, force
                })
            },
            Self::TouchpadPressure { device_id, pressure, stage } =>
                Event::new(Event::TouchpadPressure, TouchpadPressure{
                    device_id, pressure, stage
                })
                

        }
    }
}