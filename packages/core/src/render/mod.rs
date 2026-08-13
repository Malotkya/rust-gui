use std::{
    fmt,
    ops::Deref,
    rc::Rc
};
pub use vertex::{VertexData, VertexShape, Size, Topology};
pub use context::RenderContext;
pub use target::RenderTarget;
pub use err::RenderError;

pub(crate) mod context;
pub(crate) mod pipeline;
pub(crate) mod shader;
pub(crate) mod swapchain;
pub(crate) mod target;
pub(crate) mod vertex;

#[derive(Clone)]
pub(crate) struct Device(std::rc::Rc<ash::Device>);

impl Device {
    pub fn new(inner: ash::Device) -> Self {
        Self(Rc::new(inner))
    }
}

impl Deref for Device {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ash::Device")
            .field("0", &(self.0.as_ref() as *const ash::Device))
            .finish()
    }
}

pub mod ctx {
    pub use super::context::RenderContext;
    pub(crate) use super::target::DeviceContext;
}

pub mod err {
    use winit::error::OsError;
    use std::fmt;
    pub use super::context::ContextError;
    pub use super::shader::ShaderError;
    pub use super::pipeline::PipelineError;
    pub use super::swapchain::{SurfaceError, SwapchainError};
    pub use super::target::RenderTargetError;
    pub use super::vertex::GpuDataError;

    impl From<ContextError> for RenderError {
        fn from(value:ContextError) -> Self {
            Self::ContextError(value)
        }
    }

    impl From<ShaderError> for RenderError {
        fn from(value:ShaderError) -> Self {
            Self::ShaderError(value)
        }
    }

    impl From<PipelineError> for RenderError {
        fn from(value:PipelineError) -> Self {
            Self::PipelineError(value)
        }
    }

    impl From<SurfaceError> for RenderError {
        fn from(value:SurfaceError) -> Self {
            Self::SurfaceError(value)
        }
    }

    impl From<SwapchainError> for RenderError {
        fn from(value: SwapchainError) -> Self {
            Self::SwapchainError(value)
        }
    }

    impl From<RenderTargetError> for RenderError {
        fn from(value:RenderTargetError) -> Self {
            Self::RenderTargetError(value)
        }
    }

    impl From<GpuDataError> for RenderError {
        fn from(value: GpuDataError) -> Self {
            Self::GpuDataError(value)
        }
    }

    #[cfg_attr(debug_assertions, derive(Debug))]
    pub enum RenderError {
        ContextError(ContextError),
        ShaderError(ShaderError),
        PipelineError(PipelineError),
        SurfaceError(SurfaceError),
        SwapchainError(SwapchainError),
        RenderTargetError(RenderTargetError),
        GpuDataError(GpuDataError),
        OsError(OsError)
    }

    impl fmt::Display for RenderError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::ContextError(e) => write!(f, "Context Error: {}", e),
                Self::ShaderError(e) => write!(f, "Shader Error: {}", e),
                Self::PipelineError(e) => write!(f, "Pipeline Error: {}", e),
                Self::SurfaceError(e) => write!(f, "Surface Error: {}", e),
                Self::SwapchainError(e) => write!(f, "Swapchain Error: {}", e),
                Self::RenderTargetError(e) => write!(f, "RenderTarget Error: {}", e),
                Self::GpuDataError(e) => write!(f, "GpuBatch Error: {}", e),
                Self::OsError(e) => write!(f, "OS Error: {}", e)
            }   
        }
    }

    impl From<OsError> for RenderError {
        fn from(value: OsError) -> Self {
            Self::OsError(value)
        }
    }
}