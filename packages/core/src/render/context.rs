use ash::{khr, vk};
use raw_window_handle::HandleError;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};
use std::{
    rc::Rc,
    fmt
};
use crate::ApplicationInfo;
use super::err::RenderError;

#[derive(Debug)]
pub enum ContextError {
    NoVulkanLibrary,
    MissingVulkanEntryPoint,
    DisplayHandleError(HandleError),
    WindowHandleError(HandleError),
    FailedToInitInistance,
    InitSurfaceFailed,
    PhysicalDeviceNotFound
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoVulkanLibrary => write!(f, "No Vulkan library found!"),
            Self::MissingVulkanEntryPoint => write!(f, "Unable to load vulkan library!"),
            Self::DisplayHandleError(handle_error) => match handle_error {
                HandleError::NotSupported => write!(f, "Unable to load Display Handle!"),
                HandleError::Unavailable => write!(f, "Display handle is unavailable!"),
                _ => write!(f, "An unknown error occured trying to load a Display Handle!")
            },
            Self::WindowHandleError(handle_error) => match handle_error {
                HandleError::NotSupported => write!(f, "Unable to load Window Handle!"),
                HandleError::Unavailable => write!(f, "Window handle is unavailable!"),
                _ => write!(f, "An unknown error occured trying to load a Window Handle!")
            },
            Self::FailedToInitInistance => write!(f, "Failed to load Vulkan Instance!"),
            Self::InitSurfaceFailed => write!(f, "Faild to initalize Surface!"),
            Self::PhysicalDeviceNotFound => write!(f, "No compatable Physical Device is available!"),
        }
    }
}


pub struct RenderContext {
    pub(super) queue_family_index: u32,
    pub(super) surface_loader: khr::surface::Instance,
    pub(super) physical_device: vk::PhysicalDevice,
    pub(super) instance: ash::Instance,
    pub(super) entry: ash::Entry,
}

#[cfg(debug_assertions)]
impl fmt::Debug for RenderContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderContext")
            .field("queue_family_index", &self.queue_family_index)
            .field("surface_loader", &(&self.surface_loader as *const khr::surface::Instance))
            .field("physical_device", &self.physical_device)
            .field("instance", &(&self.instance as *const ash::Instance))
            .field("entry", &(&self.entry as *const ash::Entry))
            .finish()
    }
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
            .map(|slice|slice.to_vec())
            .unwrap_or(Vec::new());

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