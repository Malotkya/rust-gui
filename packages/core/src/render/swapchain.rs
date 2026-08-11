use ash::{khr, vk};
use winit::{
    dpi::PhysicalSize,
};
use std::{
    ops::Deref,
    rc::Rc
};
use super::{
    ctx::{DeviceContext, RenderContext},
    err::{ContextError, RenderError}
};

#[derive(Debug, Clone)]
pub enum SurfaceError {
    MissingFormats,
    MissingModes,
    AquireImageFailed,
    CreateImageViewFailed(usize)
}

pub struct Swapchain {
    pub extent: vk::Extent2D,
    pub image_format: vk::Format,
    pub image_views: Vec<vk::ImageView>,
    //pub images: Vec<vk::Image>,
    inner: vk::SwapchainKHR,
    device: Rc<ash::Device>,
    pub loader: khr::swapchain::Device
}

impl Deref for Swapchain {
    type Target = vk::SwapchainKHR;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Swapchain {
    pub unsafe fn new(ctx:DeviceContext, surface: &vk::SurfaceKHR, size:PhysicalSize<u32>) -> Result<Self, RenderError> {
        let loader = khr::swapchain::Device::new(&ctx.inner.instance, &ctx.device);
        
        let (swapchain_create_info, surface_format, extent)
            = ctx.inner.create_swapchain_info(*surface, size)?;

        // Create the swapchain first
        let inner = loader.create_swapchain(&swapchain_create_info, None)
            .map_err(|_|ContextError::InitSwapChainFailed)?;

        // If anything fails after this, we need manual cleanup
        // So wrap the remaining operations in a guard
        let mut swapchain = Self {
            inner,
            extent,
            image_format: surface_format.format,
            image_views: Vec::new(),
            loader,
            device: ctx.device.clone(),
        };

        // Try to get images and create views
        let images = swapchain.loader.get_swapchain_images(swapchain.inner)
            .map_err(|_|SurfaceError::AquireImageFailed)?;

        swapchain.image_views.reserve(images.len());
        for (i, img) in images.into_iter().enumerate() {
            let create_view_info = vk::ImageViewCreateInfo::default()
                .image(img)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(surface_format.format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            swapchain.image_views.push(
                ctx.device.create_image_view(&create_view_info, None)
                    .map_err(|_|SurfaceError::CreateImageViewFailed(i))?
            );
        }

        Ok(swapchain)
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            while let Some(view) = self.image_views.pop() {
                self.device.destroy_image_view(view, None);
            }
            
            if self.inner != vk::SwapchainKHR::null() {
                println!("Swapcahin: {:?}", self.inner);
                self.loader.destroy_swapchain(self.inner, None);
            }
        }
    }
}

impl RenderContext {
    pub fn create_swapchain_info(&self, surface: vk::SurfaceKHR, size:PhysicalSize<u32>) -> Result<(vk::SwapchainCreateInfoKHR<'_>, vk::SurfaceFormatKHR, vk::Extent2D), RenderError> {
        unsafe {
            let caps = self.surface_loader.get_physical_device_surface_capabilities(self.physical_device, surface)
                .map_err(|_|ContextError::InitShaderCompilerFailed)?;

            let formats = self.surface_loader.get_physical_device_surface_formats(self.physical_device, surface)
                .map_err(|_|SurfaceError::MissingFormats)?;

            let surface_format = formats
                .iter()
                .find(|f| f.format == vk::Format::B8G8R8A8_SRGB)
                .unwrap_or(&formats[0])
                .clone();

            let present_mode = self.surface_loader.get_physical_device_surface_present_modes(self.physical_device, surface)
                .map_err(|_|SurfaceError::MissingModes)?
                .iter()
                .cloned()
                .find(|&m| m == vk::PresentModeKHR::MAILBOX)
                .unwrap_or(vk::PresentModeKHR::FIFO);

            let mut image_count = caps.min_image_count + 1;
            if image_count > caps.max_image_count {
                image_count = caps.max_image_count;
            }

            let extent = if caps.current_extent.width != u32::MAX {
                caps.current_extent
            } else {
                vk::Extent2D {
                    width: size.width,
                    height: size.height,
                }
            };

            let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
                .surface(surface)
                .min_image_count(image_count)
                .image_format(surface_format.format)
                .image_color_space(surface_format.color_space)
                .image_extent(extent)
                .image_array_layers(1)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(caps.current_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true);

            Ok((swapchain_create_info, surface_format, extent))
        }
    }
}