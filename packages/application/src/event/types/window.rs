use winit::event::Ime;
use crate::event::ScrollData;

use super::{
    external::*,
    super::{Event, EventData}
};

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum WindowEvent {
    SurfaceResized(PhysicalSize<u32>),
    Moved(PhysicalPosition<i32>),
    CloseRequested,
    Destroyed,
    Focused(bool),
    Ime(Ime),
    DoubleTap(DeviceId),
    Gesture(GestureEventType),
    TouchpadPressure(TouchpadPressureEventData),
    ScaleFactorChanged(ScaleFactorChanged),
    RedrawRequested,
    MouseWheel(ScrollData)
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct TouchpadPressureEventData {
    pub device_id: DeviceId,
    pub pressure: f32,
    pub stage: i64
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct TouchData {
    pub device_id: DeviceId,
    pub phase: TouchPhase,
    pub location: PhysicalPosition<f64>,
    pub force: Option<Force>
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct ScaleFactorChanged {
    pub scale_factor: f64,
    pub inner_size_writer: InnerSizeWriter
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct AxisMotionData {
    pub device_id: DeviceId,
    pub axis: AxisId,
    pub value: f64
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub struct GestureEventData<T> {
    pub device_id: DeviceId,
    pub delta: T,
    pub phase: TouchPhase
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum GestureEventType {
    Hold(GestureEventData<()>),
    Pinch(GestureEventData<f64>),
    Pan(GestureEventData<PhysicalPosition<f32>>),
    Rotation(GestureEventData<f32>)
}

impl Into<Event> for WindowEvent {
    fn into(self) -> Event {
        match self {
            Self::SurfaceResized(data) =>
                Event::new(Event::Resized, data),
            Self::Moved(data) =>
                Event::new(Event::Moved, data),
            Self::CloseRequested =>
                Event::new(Event::CloseRequested, ()),
            Self::Destroyed =>
                Event::new(Event::Destoryed, ()),
            Self::Focused(focus) => if focus {
                Event::new(Event::Focus, ())
            } else {
                Event::new(Event::Blur, ())
            },
            Self::Ime(_ime) => todo!("IME -> input"),
            Self::DoubleTap(data) =>
                Event::new(Event::DoubleTap, data),
            Self::Gesture(g) => match g {
                GestureEventType::Hold(data) =>
                    Event::new(Event::HoldGesture, data),
                GestureEventType::Pan(data) =>
                    Event::new(Event::PanGesture, data),
                GestureEventType::Pinch(data) => 
                    Event::new(Event::PinchGesture, data),
                GestureEventType::Rotation(data) =>
                    Event::new(Event::RotationGesture, data)
            },
            Self::TouchpadPressure(data) =>
                Event::new(Event::TouchpadPressure, data),
            Self::ScaleFactorChanged(data) =>
                Event::new(Event::ScaleFactorChanged, data),
            Self::RedrawRequested =>
                Event::new(Event::RedrawRequested, ()),
            Self::MouseWheel(data)
                => Event::new(Event::MouseWheel, data)
        }
    }
}

#[allow(non_upper_case_globals)]
impl Event {
    pub(crate) fn is_window_event(&self) -> bool {
        match self.type_name.as_str() {
            Self::Resized => true,
            Self::Moved => true,
            Self::AxisMotion => true,
            Self::CloseRequested => true,
            Self::Destoryed => true,
            //Self::Focus => true,
            //Self::Blur => true,
            //Self::DoubleTap => true,
            Self::HoldGesture => true,
            Self::PanGesture => true,
            Self::PinchGesture => true,
            Self::RotationGesture => true,
            Self::ScaleFactorChanged => true,
            Self::Drop => true,
            Self::Dragged => true,
            Self::DraggedEnd => true,
            Self::RedrawRequested => true,
            Self::ThemeChanged => true,
            Self::TouchpadPressure => true,
            //Self::MouseWheel => true,
            _ => false
        }
    }

    pub const Resized:&'static str = "resized";
    pub const Moved:&'static str = "moved";
    pub const AxisMotion:&'static str = "axis";
    pub const CloseRequested:&'static str = "close";
    pub const Destoryed:&'static str = "destroyed";
    pub const Focus:&'static str = "focus";
    pub const Blur:&'static str = "blur";
    pub const DoubleTap:&'static str = "doubletap";
    pub const HoldGesture:&'static str = "hold";
    pub const PanGesture:&'static str = "pan";
    pub const PinchGesture:&'static str = "pinch";
    pub const RotationGesture:&'static str = "rotate";
    pub const ScaleFactorChanged:&'static str = "scalefactor";
    pub const Drop:&'static str = "drop";
    pub const Dragged:&'static str = "dragged";
    pub const DraggedEnd:&'static str = "draggedend";
    pub const RedrawRequested:&'static str = "redraw";
    pub const ThemeChanged:&'static str = "themechanged";
    pub const Touch:&'static str = "touch";
    pub const TouchpadPressure:&'static str = "touchpad";
    pub const MouseWheel:&'static str = "mousewheel";
}

impl TryFrom<Event> for WindowEvent {
    type Error = Event;

    fn try_from(value:Event) -> Result<Self, Self::Error> {
        match value.type_name.as_str() {
            Event::Resized => value.deconstruct::<PhysicalSize<u32>>()
                .map(|EventData{detail, ..}|Self::SurfaceResized(detail)),
            Event::Moved => value.deconstruct::<PhysicalPosition<i32>>()
                .map(|EventData{detail, ..}|Self::Moved(detail)),
            Event::CloseRequested => Ok(
                Self::CloseRequested
            ),
            Event::Destoryed => Ok(
                Self::Destroyed
            ),
            Event::Focus => Ok(
                Self::Focused(true)
            ),
            Event::Blur => Ok(
                Self::Focused(false)
            ),
            Event::DoubleTap => value.deconstruct::<DeviceId>()
                .map(|EventData{detail, ..}|Self::DoubleTap(detail)),
            Event::HoldGesture => value.deconstruct::<GestureEventData<()>>()
                .map(|EventData{detail, ..}|Self::Gesture(
                    GestureEventType::Hold(detail)
                )),
            Event::PanGesture => value.deconstruct::<GestureEventData<PhysicalPosition<f32>>>()
                .map(|EventData{detail, ..}|Self::Gesture(
                    GestureEventType::Pan(detail)
                )),
            Event::PinchGesture => value.deconstruct::<GestureEventData<f64>>()
                .map(|EventData{detail, ..}|Self::Gesture(
                    GestureEventType::Pinch(detail)
                )),
            Event::RotationGesture => value.deconstruct::<GestureEventData<f32>>()
                .map(|EventData{detail, ..}|Self::Gesture(
                    GestureEventType::Rotation(detail)
                )),
            Event::TouchpadPressure => value.deconstruct::<TouchpadPressureEventData>()
                .map(|EventData{detail, ..}|Self::TouchpadPressure(detail)),
            Event::ScaleFactorChanged => value.deconstruct::<ScaleFactorChanged>()
                .map(|EventData{detail, ..}|Self::ScaleFactorChanged(detail)),
            Event::RedrawRequested => Ok(
                Self::RedrawRequested
            ),
            Event::MouseWheel => value.deconstruct::<ScrollData>()
                .map(|EventData{detail, ..}|Self::MouseWheel(detail)),
            _ => Err(value)
        }
    }
}

