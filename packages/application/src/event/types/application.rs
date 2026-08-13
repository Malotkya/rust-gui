use super::{
    external::*,
    super::{Event, EventData}
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct ActivationTokenData {
    pub serial: AsyncRequestSerial,
    pub token: ActivationToken
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct DeviceMotionData {
    pub axis: AxisId,
    pub value: f64
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ApplicationEvent {
    ActivationTokenDone(ActivationTokenData),
    PointerMotion(PhysicalPosition<f64>),
    ButtonUp(ButtonId),
    ButtonDown(ButtonId),
    KeyPressed(RawKeyData),
    Motion(DeviceMotionData)
}

impl Into<Event> for ApplicationEvent {
    fn into(self) -> Event {
        match self {
            Self::ActivationTokenDone(data)
                => Event::new(Event::ActivationTokenDone, data),
            Self::PointerMotion(data)
                => Event::new(Event::PointerMove, data),
            Self::ButtonDown(data)
                => Event::new(Event::ButtonDown, data),
            Self::ButtonUp(data)
                => Event::new(Event::ButtonUp, data),
            Self::KeyPressed(data)
                => Event::new(Event::KeyPressed, data),
            Self::Motion(data)
                => Event::new(Event::DeviceMotion, data)
        }
    }
}

#[allow(non_upper_case_globals)]
impl Event {
    pub(crate) fn is_application_event(&self) -> bool {
        match self.type_name.as_str() {
            Self::ActivationTokenDone => true,
            Self::PointerMove => true,
            Self::ButtonDown => true,
            Self::ButtonUp => true,
            //Self::KeyPressed => true,
            Self::DeviceMotion => true,
            Self::DeviceAdded => true,
            Self::DeviceRemoved => true,
            Self::ModifiersChanged => true,
            _ => false
        }
    }

    pub const ActivationTokenDone:&'static str = "activation-token-done";
    pub const PointerMove:&'static str = "pointermove";
    pub const ButtonDown:&'static str = "buttondown";
    pub const ButtonUp:&'static str = "buttonup";
    pub const KeyPressed:&'static str = "keypressed";
    pub const DeviceMotion:&'static str = "devicemotion";
    pub const DeviceAdded:&'static str = "deviceadded";
    pub const DeviceRemoved:&'static str = "deviceremoved";
    pub const ModifiersChanged:&'static str = "modifierschanged";
    pub const Maximized:&'static str = "maximized";
    pub const Minimized:&'static str = "minimized";
}

impl TryFrom<Event> for ApplicationEvent {
    type Error = Event;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value.type_name.as_str() {
            Event::ActivationTokenDone => value.deconstruct::<ActivationTokenData>()
                .map(|EventData{detail, ..}|Self::ActivationTokenDone(detail)),
            Event::PointerMove => value.deconstruct::<PhysicalPosition<f64>>()
                .map(|EventData{detail, ..}|Self::PointerMotion(detail)),
            Event::ButtonDown => value.deconstruct::<ButtonId>()
                .map(|EventData{detail, ..}|Self::ButtonDown(detail)),
            Event::ButtonUp => value.deconstruct::<ButtonId>()
                .map(|EventData{detail, ..}|Self::ButtonUp(detail)),
            Event::KeyPressed => value.deconstruct::<RawKeyData>()
                .map(|EventData{detail, ..}|Self::KeyPressed(detail)),
            Event::DeviceMotion => value.deconstruct::<DeviceMotionData>()
                .map(|EventData{detail, ..}|Self::Motion(detail)),
            _ => Err(value)
        }
    }
}