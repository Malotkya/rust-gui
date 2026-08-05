use ash::{khr, vk};
use std::sync::Arc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;
use super::{VulkanError, Instance};


impl Instance {
    pub fn create_surface(&self, window:&Arc<Window>) -> Result<Surface<'_>, VulkanError> {
        let window_handle  = window.window_handle()
            .map_err(|e|VulkanError::WindowHandleError(e))?;

        let display_handle = window.display_handle()
            .map_err(|e|VulkanError::DisplayHandleError(e))?;

        //SAFETY: lifetime reference will make sure surface is dropped before surface loader.
        let inner = unsafe {
            ash_window::create_surface(
                &self.instance.entry,
                &self.instance.inner,
                display_handle.as_raw(),
                window_handle.as_raw(),
                None,
            )
        }.map_err(|_|VulkanError::InitSurfaceFailed)?;

        // Safety Check: Make sure formats is dropped before inner.
        let formats = unsafe {
            self.inner.get_physical_device_surface_formats(self.physical_device, inner)
        }   .map_err(|_|VulkanError::MissingSurfaceFormats)?
            .into_iter().next()
            .ok_or_else(||VulkanError::MissingSurfaceFormats)?;

        // Safety Check: Make sure capabilites is dropped before inner.
        let capabilites = unsafe {
            self.inner.get_physical_device_surface_capabilities(self.physical_device, inner)
        }.map_err(|_|VulkanError::MissingSurfaceCapabilities)?;

        Ok(Surface {
            loader: &self.surface_loader,
            formats,
            capabilites,
            inner
        })
    }

    pub fn destory_surface(&self, surface:&Surface<'_>) {
        //SAFETY: Drop Attribute will make sure that all surfaces are destroyed before being dropped
        unsafe {
            self.surface_loader.destroy_surface(surface.inner, None);
        }
    }
}

pub struct Surface<'L> {
    pub formats: vk::SurfaceFormatKHR,
    pub capabilites: vk::SurfaceCapabilitiesKHR,
    pub(crate) inner: vk::SurfaceKHR,
    loader:&'L khr::surface::Instance
}

impl<'a> Drop for Surface<'a> {
    fn drop(&mut self) {
        self.loader.destory_surface(&self.inner, None);
    }
}

