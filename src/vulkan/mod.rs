use raw_window_handle::HandleError;

mod device;
pub use device::*;
mod info;
pub use info::*;
mod instance;
pub use instance::*;
mod surface;
pub use surface::*;
mod swapchain;
pub use swapchain::*;

pub enum VulkanError {
    NoVulkanLibrary,
    MissingVulkanEntryPoint,
    FailedToInit,
    DisplayHandleError(HandleError),
    WindowHandleError(HandleError),
    MissingExtensionRequirements,
    PhysicalDeviceNotFound,
    UnableToFindQueue,
    MissingQueueFamilyIndex,
    InitLogicDeviceFailed,
    InitSurfaceFailed,
    MissingSurfaceFormats,
    MissingSurfaceCapabilities,
    InitSwapChainFailed,
    AquireImageFailed,
    InitSemaphoreFailed,
    InitCommandPoolFailed,
    AllocateCommandBufferFailed,
    SubmitQueueFailed
}