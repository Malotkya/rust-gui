use ash::{khr, vk};
use raw_window_handle::HandleError;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};
use std::rc::Rc;
use crate::ApplicationInfo;
use super::err::RenderError;

#[derive(Debug)]
pub enum ContextError {
    NoVulkanLibrary,
    MissingVulkanEntryPoint,
    DisplayHandleError(HandleError),
    WindowHandleError(HandleError),
    MissingExtensionRequirements,
    FailedToInitInistance,
    InitSurfaceFailed,
    PhysicalDeviceNotFound,
    InitLogicDeviceFailed,
    InitSwapChainFailed,
    AquireImageFailed,
    InitShaderCompilerFailed,
    InitRenderPassFailed,
    InitFrameBufferFailed(usize),
    InitCommandPoolFailed,
    InitCommandBufferFailed,
    InitSemaphoreFailed,
    InitFenceFailed
}


pub struct RenderContext {
    pub(super) queue_family_index: u32,
    pub(super) surface_loader: khr::surface::Instance,
    pub(super) physical_device: vk::PhysicalDevice,
    pub(super) instance: ash::Instance,
    pub(super) entry: ash::Entry,
}

impl RenderContext {
    pub fn new(app_info:&ApplicationInfo, window:&Window) -> Result<Rc<Self>, RenderError> {
        //SAFETY: make sure the entry is dropped last.
        let entry = unsafe { ash::Entry::load() }
            .map_err(|e|match e {
                ash::LoadingError::LibraryLoadFailure(_) => ContextError::NoVulkanLibrary,
                ash::LoadingError::MissingEntryPoint(_) => ContextError::MissingVulkanEntryPoint
            })?;

        let display_handle = window.display_handle()
            .map_err(|e|ContextError::DisplayHandleError(e))?
            .as_raw();

        let mut extensions = ash_window::enumerate_required_extensions(display_handle)
            .map_err(|_|ContextError::MissingExtensionRequirements)?
            .to_vec();

        //TODO: check for any required extensions needed by later code?
        extensions.push(khr::surface::NAME.as_ptr());

        //SAFTEY: make sure instance is dropped before entry.
        let instance = unsafe { entry.create_instance(
            &vk::InstanceCreateInfo::default()
                .application_info(&app_info.vk_info())
                .enabled_extension_names(&extensions),
            None
        ) }.map_err(|_|ContextError::FailedToInitInistance)?;

        let surface_loader = khr::surface::Instance::new(&entry, &instance);

        let window_handle = window.window_handle()
            .map_err(|e|ContextError::WindowHandleError(e))?
            .as_raw();

        //SAFETY: Dropped at end of function
        let surface = unsafe {
            ash_window::create_surface(&entry, &instance, display_handle, window_handle, None)
                .map_err(|_|ContextError::InitSurfaceFailed)?
        };

        //SAFTEY: make sure physical_device is dropped before instance.
        let (physical_device, qfi) = unsafe { instance.enumerate_physical_devices() }
            .map_err(|_|ContextError::PhysicalDeviceNotFound)?
            .into_iter()
            .filter_map(|device| unsafe { instance.get_physical_device_queue_family_properties(device) }
                .into_iter()
                .enumerate()
                .find_map(|(i, q)| unsafe { surface_loader.get_physical_device_surface_support(device, i as u32, surface) }
                    .ok().and_then(|surface_suport|(q.queue_flags.contains(vk::QueueFlags::GRAPHICS) && surface_suport)
                        .then_some((device, i))
                ))
            ).min_by_key(|(device, _)| 
                match unsafe { instance.get_physical_device_properties(*device) }.device_type {
                    vk::PhysicalDeviceType::DISCRETE_GPU => 0,
                    vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                    vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
                    vk::PhysicalDeviceType::CPU => 3,
                    vk::PhysicalDeviceType::OTHER => 4,
                    _ => 5
                }
            ).ok_or_else(||ContextError::PhysicalDeviceNotFound)?;

        // No Longer using this surface.
        unsafe {
            surface_loader.destroy_surface(surface, None);
        }

        Ok(Rc::new(Self {
            entry,
            physical_device,
            instance,
            queue_family_index: qfi as u32,
            surface_loader
        }))
    }

    pub fn find_memory_type(&self, type_filter: u32, properties: vk::MemoryPropertyFlags) -> Option<u32> {
        let mem_properties = unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };

        for (i, mt) in mem_properties.memory_types.iter().enumerate() {
            if (type_filter & (1 << i)) != 0 && mt.property_flags.contains(properties) {
                return Some(i as u32);
            }
        }

        None
    }
}

impl Drop for RenderContext {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}