use std::{
    ffi::CStr,
    marker::PhantomData
};
use ash::vk;

const CRATE_NAME_BYTES: &'static [u8] = concat!(env!("CARGO_CRATE_NAME"), "\0").as_bytes();
const CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub trait ApiVersion {
    fn as_api(&self) -> u32;
}

impl ApiVersion for (u32, u32, u32, u32) {
    fn as_api(&self) -> u32 {
        vk::make_api_version(self.0, self.1, self.2, self.3)
    }
}

impl ApiVersion for u32 {
    fn as_api(&self) -> u32 {
        *self
    }
}

impl ApiVersion for &str {
    fn as_api(&self) -> u32 {
        let mut it = self.split('.')
            .filter_map(|str|str.parse::<u32>().ok())
            .rev();
        
        let patch = it.next().unwrap_or(0);
        let minor = it.next().unwrap_or(0);
        let major = it.next().unwrap_or(0);
        let variant = it.next().unwrap_or(0);

        vk::make_api_version(variant, major, minor, patch)
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ApplicationInfo<'a> {
    pub name:&'a CStr,
    pub api_version: u32,
}

impl<'a> Default for ApplicationInfo<'a> {
    fn default() -> Self {
        Self { name: c"", api_version:0}
    }
}

impl<'a> ApplicationInfo<'a> {
    pub(crate) fn vk_info(&self) -> vk::ApplicationInfo<'a> {
        vk::ApplicationInfo{
            s_type: vk::StructureType::default(),
            p_next: std::ptr::null(),
            p_application_name: self.name.as_ptr(),
            application_version: self.api_version,
            p_engine_name: CRATE_NAME_BYTES.as_ptr() as *const i8,
            engine_version: CRATE_VERSION.as_api(),
            api_version: vk::API_VERSION_1_3,
            _marker: PhantomData::default()
        }
    }
}