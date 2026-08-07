use ash::vk;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

impl<T: Into<f32> + Clone> From<&[T]> for Position {
    fn from(value: &[T]) -> Self {
        let mut it = value.iter();
        let x = it.next()
            .map(|v|v.clone().into())
            .unwrap_or(0f32);
        let y = it.next()
            .map(|v|v.clone().into())
            .unwrap_or(0f32);

        Self{x, y}
    }
}

impl<T: Into<f32>> From<[T; 2]> for Position {
    fn from(value: [T; 2]) -> Self {
        let [x, y] = value;

        Self {
            x: x.into(),
            y: y.into()
        }
    }
}

impl Position {
    pub fn new<X: Into<f32>, Y:Into<f32>>(x:X, y:Y) -> Self {
        Self {
            x: x.into(),
            y: y.into()
        }
    }
    pub(crate) fn binding() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride((std::mem::size_of::<f32>() * 2) as u32)
    }

    pub(crate) fn attribute() -> vk::VertexInputAttributeDescription{
        vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT) 
            .offset(0)
    }
}