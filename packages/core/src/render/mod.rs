pub use vertex::VertexShape;

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
    pub use super::context::ContextError;
    pub use super::shader::ShaderError;
    pub use super::pipeline::PipelineError;
    pub use super::swapchain::SurfaceError;
    pub use super::target::RenderTargetError;
}