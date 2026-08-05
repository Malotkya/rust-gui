use std::{cell::LazyCell, marker::PhantomData};
use ash::{khr, vk};
use raw_window_handle::HasDisplayHandle;
use winit::window::Window;
use super::{ApplicationInfo, VulkanError, Device};

const ENTRY:LazyCell<Result<ash::Entry, ash::LoadingError>> = LazyCell::new(||unsafe {
    ash::Entry::load()
});

fn get_entry() -> Result<ash::Entry, VulkanError> {
    ENTRY.as_ref()
        .map(|e|e.clone())
        .map_err(|e|match e {
            ash::LoadingError::LibraryLoadFailure(_) => VulkanError::NoVulkanLibrary,
            ash::LoadingError::MissingEntryPoint(_) => VulkanError::MissingVulkanEntryPoint
        })
}

pub struct Instance{
    pub surface_loader: khr::surface::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub(crate) inner: ash::Instance,
    pub(crate) entry: ash::Entry,
}

impl Instance {
    pub fn new(info:ApplicationInfo<'_>, window:&Window) -> Result<Self, VulkanError> {
        let entry = get_entry()?;

        let display_handle = window.display_handle()
            .map_err(|e|VulkanError::DisplayHandleError(e))?
            .as_raw();

        let mut extensions = ash_window::enumerate_required_extensions(display_handle)
            .map_err(|_|VulkanError::MissingExtensionRequirements)?
            .to_vec();

        extensions.push(khr::surface::NAME.as_ptr());

        // SAFTEY: Using phantom markers to make sure everything created with instance is dropped first.
        let inner = unsafe { entry.create_instance(
            &vk::InstanceCreateInfo::default()
                .application_info(&info.vk_info())
                .enabled_extension_names(&extensions),
            None
        ) }.map_err(|_|VulkanError::FailedToInit)?;

        //SAFTEY: make sure physical_device is dropped before inner.
        let physical_device = unsafe { inner.enumerate_physical_devices() }
            .map_err(|_|VulkanError::PhysicalDeviceNotFound)?
            .into_iter()
            .min_by_key(|device| match unsafe { inner.get_physical_device_properties(*device) }.device_type
            {
                vk::PhysicalDeviceType::DISCRETE_GPU => 0,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
                vk::PhysicalDeviceType::CPU => 3,
                vk::PhysicalDeviceType::OTHER => 4,
                _ => 5
            }).ok_or_else(||VulkanError::PhysicalDeviceNotFound)?;

        let surface_loader = khr::surface::Instance::new(&entry, &inner);

        Ok(Self{
            inner, entry,
            surface_loader,
            physical_device
        })
    }

    pub fn get_device<T: vk::ExtendsDeviceCreateInfo + ?Sized>(&self, queue_flags:vk::QueueFlags, mut device_rendering_features:T) -> Result<Device<'_>, VulkanError> {
        let queue_family_index = unsafe { self.inner.get_physical_device_queue_family_properties(self.physical_device) }
            .iter()
            .enumerate()
            .find_map(|(idx, prop)| prop.queue_flags.contains(queue_flags)
                .then_some(idx as u32)
            ).ok_or_else(||VulkanError::MissingQueueFamilyIndex)?;
        
        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0]);

        let extension_names = [khr::swapchain::NAME.as_ptr()];
        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_create_info))
            .enabled_extension_names(&extension_names)
            .push_next(&mut device_rendering_features);

        //SAFETY: Lifetime reference will make sure that logical device is dropped before the physical device.
        let inner = unsafe {
            self.inner.create_device(self.physical_device, &device_create_info, None)
        }.map_err(|_|VulkanError::InitLogicDeviceFailed)?;

        let swapchain_loader = khr::swapchain::Device::new(&self.inner, &inner);

        Ok(Device {
            inner,
            queue_family_index,
            swapchain_loader,
            _marker: PhantomData::default()
        })
    }
}


