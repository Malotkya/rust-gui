use ash::vk;
use super::{Device, VulkanError, Queue};

impl<'a> Device<'a> {
    pub fn create_swapchain(&self, create_info: vk::SwapchainCreateInfoKHR) -> Result<SwapChain, VulkanError> {
        // Safety Check: Lifetime reference to context makes sure that swapchain can't be dropped before context
        unsafe {
            let inner = self.inner.create_swapchain(&create_info, None)
                .map_err(|_|VulkanError::InitSwapChainFailed)?;

            let images = self.inner.get_swapchain_images(inner)
                .map_err(|_|VulkanError::InitSwapChainFailed)?;

            let views = images.iter().map(|img|{
                let create_info = vk::ImageViewCreateInfo::default()
                    .image(img)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(create_info.image_format)
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        level_count: 1,
                        layer_count: 1,
                        ..Default::default()}
                    );
                    
                    self.device.create_image_view(&create_info, None).unwrap()
            });

            Ok(SwapChain{
                inner, images,
                views,
                device: &self
            })
        }
    }

    pub fn destory_swapchain(&self, swapchain:&SwapChain) {
        //SAFETY: Drop Attribute will make sure that all surfaces are destroyed before being dropped
        unsafe {
            for img in &swapchain.images {
                self.device.destroy_image(*img, None);
            }

            for view in &swapchain.views {
                self.device.destroy_image_view(*view, None);
            }

            self.inner.destroy_swapchain(swapchain.inner, None);
        }
    }
}

pub struct SwapChain<'L> {
    pub(crate) inner: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub views: Vec<vk::ImageView>,
    device:&'L Device<'L>
}

impl<'a> SwapChain<'a> {
    pub fn update(&mut self, update_info: vk::SwapchainCreateInfoKHR) -> Result<(), VulkanError> {
        //SAFETY: Lifetime reference will make sure loader still exists.
        unsafe {
            let new_inner = self.loader.inner.create_swapchain(
                &update_info
                    .old_swapchain(self.inner), 
                None
            ).map_err(|_|VulkanError::InitSwapChainFailed)?;

            self.device.swapchain_loader.destroy_swapchain(self.inner, None);
            self.inner = new_inner;
        }

        Ok(())
    }

    pub fn acquire_next_image(&self, timeout:u64, semaphore: vk::Semaphore) -> Result<(u32, bool), VulkanError> {
        unsafe {
                self.device.swapchain_loader.acquire_next_image(
                self.inner,
                timeout,
                semaphore,
                vk::Fence::null()
            ).map_err(|_|VulkanError::AquireImageFailed)
        }
    }

    pub fn queue_present(&self, queue: Queue<'_>, present_info: &vk::PresentInfoKHR) -> Result<bool, VulkanError> {
        unsafe {
            self.device.swapchain_loader.queue_present(queue.inner, present_info)
                .map_err(|_|VulkanError::SubmitQueueFailed)
        }
    }
}

impl<'a> Drop for SwapChain<'a> {
    fn drop(&mut self) {
        self.device.destory_swapchain(self);
    }
} 