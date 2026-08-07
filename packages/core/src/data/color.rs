use ash::vk;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl<T: Into<u8> + Clone> From<&[T]> for Color {
    fn from(value: &[T]) -> Self {
        let mut it = value.iter();
        let red = it.next()
            .map(|v|v.clone().into())
            .unwrap_or(0);
        let green = it.next()
            .map(|v|v.clone().into())
            .unwrap_or(0);
        let blue = it.next()
            .map(|v|v.clone().into())
            .unwrap_or(0);
        let alpha = it.next()
            .map(|v|v.clone().into())
            .unwrap_or(255);

        Self{red, green, blue, alpha}
    }
}

impl<T: Into<u8>> From<[T; 3]> for Color {
    fn from(value: [T; 3]) -> Self {
        let [r, g, b] = value;

        Self {
            red: r.into(),
            green: g.into(),
            blue: b.into(),
            alpha: 255
        }
    }
}

impl<T: Into<u8>> From<[T; 4]> for Color {
    fn from(value: [T; 4]) -> Self {
        let [r, g, b, a] = value;

        Self {
            red: r.into(),
            green: g.into(),
            blue: b.into(),
            alpha: a.into()
        }
    }
}

impl Color {
    pub fn new<R: Into<u8>, G: Into<u8>, B: Into<u8>>(red:R, green:G, blue:B) -> Self {
        Self {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
            alpha: 255
        }
    }

    pub fn new_alpha<R: Into<u8>, G: Into<u8>, B: Into<u8>, A:Into<u8>>(red:R, green:G, blue:B, alpha:A) -> Self {
        Self {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
            alpha: alpha.into()
        }
    }

    pub fn float32(&self) -> [f32; 4] {
        [
            self.red.into(),
            self.green.into(),
            self.blue.into(),
            self.alpha.into()
        ]
    }

    pub(crate) fn binding() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::default()
        .binding(1)
        .stride(4) 
        .input_rate(vk::VertexInputRate::INSTANCE)
    }

    pub(crate) fn attribute() -> vk::VertexInputAttributeDescription{
        vk::VertexInputAttributeDescription::default()
            .binding(1)
            .location(1)
            .format(vk::Format::R8G8B8A8_UNORM)
            .offset(0)
    }
}