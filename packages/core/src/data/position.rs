use crate::render::vertex::Size;
use ash::vk;

#[derive(Clone, Copy, Debug)]
pub enum Position {
    Coordinate(i32, i32),
    Reference(f32, f32)
}

impl Position {
    pub fn new_coordinate<X: Into<i32>, Y: Into<i32>>(x:X, y:Y) -> Self {
        Self::Coordinate(x.into(), y.into())
    }

    pub fn new_reference<X: Into<f32>, Y:Into<f32>>(x:X, y:Y) -> Self {
        Self::Reference(x.into(), y.into())
    }

    pub fn format_ref(&self, extent:&Size) -> GpuVertexPosition {
        match self {
            Self::Reference(x, y) => GpuVertexPosition {
                x: *x,
                y: *y
            },
            Self::Coordinate(x, y) => GpuVertexPosition {
                x: ((*x as f32) / extent.width)  * 2.0 - 1.0,
                y: 1.0 - ((*y as f32) / extent.height) * 2.0
            }
        }
    }

    pub fn format_coord(&self, extent:&Size) -> GpuVertexPosition {
        match self {
            Self::Reference(x, y) => GpuVertexPosition {
                x: (x + 1.0) * 0.5 * extent.width,
                y: (1.0 - y) * 0.5 * extent.height,
            },
            Self::Coordinate(x, y) => GpuVertexPosition {
                x: (*x as f32),
                y: (*y as f32)
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GpuVertexPosition {
    pub x: f32,
    pub y: f32
}

impl GpuVertexPosition {
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

    pub fn add(&self, other:&Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    pub fn sub(&self, other:&Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    pub fn mul(&self, other:&Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

    pub fn div(&self, other:&Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }

    pub fn to_coordinate(&self, extent:&Size) -> Self {
        Self {
            x: (self.x + 1.0) * 0.5 * extent.width,
            y: (1.0 - self.y) * 0.5 * extent.height,
        }
    }

    pub fn from_cordinate(&self, extent:&Size) -> Self {
        Self {
            x: (self.x / extent.width)  * 2.0 - 1.0,
            y: 1.0 - (self.y / extent.height) * 2.0
        }
    }
}

macro_rules! into_position {
    (Coordinate: $($type:ty),+ ) => {
        $(
            impl From<&[$type]> for Position {
                fn from(value: &[$type]) -> Self {
                    let mut it = value.iter();
                    let x = it.next()
                        .map(|n| *n as i32)
                        .unwrap_or(0);
                    let y = it.next()
                        .map(|n| *n as i32)
                        .unwrap_or(0);

                    Self::Coordinate(x, y)
                }
            }

            impl<const N:usize> From<[$type; N]> for Position {
                fn from(value: [$type; N]) -> Self {
                    (&value as &[$type]).into()
                }
            }
        )+
    };
    (Reference: $($type:ty),+ ) => {
        $(
            impl From<&[$type]> for Position {
                fn from(value: &[$type]) -> Self {
                    let mut it = value.iter();
                    let x = it.next()
                        .map(|n| *n as f32)
                        .unwrap_or(0.0);
                    let y = it.next()
                        .map(|n| *n as f32)
                        .unwrap_or(0.0);

                    Self::Reference(x, y)
                }
            }

            impl<const N:usize> From<[$type; N]> for Position {
                fn from(value: [$type; N]) -> Self {
                    (&value as &[$type]).into()
                }
            }
        )+
    };
}

into_position!(Coordinate: u8, u16, u32, u64, i8, i16, i64);
into_position!(Reference: f64);