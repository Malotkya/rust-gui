use ash::{vk, khr};
use std::marker::PhantomData;
use super::VulkanError;

pub struct Device<'I> {
    pub(crate) swapchain_loader: khr::swapchain::Device,
    pub(crate) inner: ash::Device,
    pub(crate) queue_family_index: u32,
    pub(crate) _marker: PhantomData<&'I ()>
}

impl<'a> Device<'a> {
    pub fn get_queue(&self) -> Queue<'_> {
        //SAFETY: Make sure queue is dropped before device.
        unsafe {
            let inner = self.inner.get_device_queue(self.queue_family_index, 0);
            Queue{
                inner,
                _marker: PhantomData::default()
            }
        }
    }

    pub fn create_semaphore(&self, info:&vk::SemaphoreCreateInfo) -> Result<(vk::Semaphore, vk::Semaphore), VulkanError> {
        unsafe { Ok((
            self.inner.create_semaphore(info, None)
                .map_err(|_|VulkanError::InitSemaphoreFailed)?,
            self.inner.create_semaphore(info, None)
                .map_err(|_|VulkanError::InitSemaphoreFailed)?
        )) }
    }

    pub unsafe fn create_command_pool(&self, pool_create_flags:vk::CommandPoolCreateFlags) -> Result<vk::CommandPool, VulkanError> {
        let pool_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(self.queue_family_index)
            .flags(pool_create_flags);

        unsafe {
            self.inner.create_command_pool(&pool_info, None)
        }.map_err(|e|VulkanError::InitCommandPoolFailed)
    }

    pub fn destory_command_pool(&self, command_pool:vk::CommandPool) {
        unsafe {
            self.inner.destroy_command_pool(command_pool.inner, None);
        }
    }

    /// SAFETY: Make sure to use free_command_buffers when done
    pub unsafe fn allocate_command_buffer(&self, alloc_info: &vk::CommandBufferAllocateInfo<'_>) -> Result<Vec<vk::CommandBuffer>, VulkanError> {
        unsafe {
            self.inner.allocate_command_buffers(alloc_info)
        }.map_err(|_|VulkanError::AllocateCommandBufferFailed)
    }

    pub fn free_command_buffers(&self, command_pool:vk::CommandPool, buffers:&[vk::CommandBuffer]) {
        unsafe {
            self.inner.free_command_buffers(command_pool, buffers)
        };
    }
}

pub struct Queue<'D> {
    pub(crate) inner: vk::Queue,
    _marker: PhantomData<&'D Device<'D>>
}