pub use vertex::VertexShape;
pub use context::RenderContext;
pub use target::RenderTarget;
pub use err::RenderError;

pub(crate) mod context;
pub(crate) mod pipeline;
pub(crate) mod shader;
pub(crate) mod swapchain;
pub(crate) mod target;
pub(crate) mod vertex;

pub mod ctx {
    pub use super::context::RenderContext;
    pub(crate) use super::target::DeviceContext;
}

pub mod err {
    use winit::error::OsError;
    pub use super::context::ContextError;
    pub use super::shader::ShaderError;
    pub use super::pipeline::PipelineError;
    pub use super::swapchain::SurfaceError;
    pub use super::target::RenderTargetError;

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

    impl From<RenderTargetError> for RenderError {
        fn from(value:RenderTargetError) -> Self {
            Self::RenderTargetError(value)
        }
    }

    #[derive(Debug)]
    pub enum RenderError {
        ContextError(ContextError),
        ShaderError(ShaderError),
        PipelineError(PipelineError),
        SurfaceError(SurfaceError),
        RenderTargetError(RenderTargetError),
        OsError(OsError)
    }

    impl From<OsError> for RenderError {
        fn from(value: OsError) -> Self {
            Self::OsError(value)
        }
    }
}