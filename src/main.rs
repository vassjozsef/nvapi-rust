#![allow(non_camel_case_types)]

pub type NvAPI_Status = i32;
pub type NvU32 = u32;
pub const NVAPI_OK: NvAPI_Status = 0;
pub const NVAPI_SHORT_STRING_MAX: usize = 64;
pub const NVAPI_MAX_PHYSICAL_GPUS: usize = 64;
pub type NvAPI_ShortString = [u8; NVAPI_SHORT_STRING_MAX];
pub type NV_SYSTEM_TYPE = i32;
pub type NV_GPU_TYPE = i32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct NvPhysicalGpuHandle(*const ::std::os::raw::c_void);

impl Default for NvPhysicalGpuHandle {
    fn default() -> Self {
        NvPhysicalGpuHandle(::std::ptr::null())
    }
}

#[link(name = "nvapi64", kind = "static")]
extern "stdcall" {
    pub fn NvAPI_Initialize() -> NvAPI_Status;
    pub fn NvAPI_GetInterfaceVersionString(szDesc: *mut NvAPI_ShortString) -> NvAPI_Status;
    pub fn NvAPI_EnumPhysicalGPUs(
        nvGPUHandle: *mut [NvPhysicalGpuHandle; NVAPI_MAX_PHYSICAL_GPUS],
        pGpuCount: *mut NvU32,
    ) -> NvAPI_Status;
    pub fn NvAPI_GPU_GetSystemType(
        hPhysicalGpu: NvPhysicalGpuHandle,
        pSystemType: *mut NV_SYSTEM_TYPE,
    ) -> NvAPI_Status;
    pub fn NvAPI_GPU_GetGPUType(
        hPhysicalGpu: NvPhysicalGpuHandle,
        pGpuType: *mut NV_GPU_TYPE,
    ) -> NvAPI_Status;
    pub fn NvAPI_GPU_GetFullName(
        hPhysicalGpu: NvPhysicalGpuHandle,
        szName: *mut NvAPI_ShortString,
    ) -> NvAPI_Status;
}

fn main() {
    let ret = unsafe { NvAPI_Initialize() };
    if ret != NVAPI_OK {
        println!["Failed to NvAPI_Initialize: {}", ret];
    }

    let mut version: NvAPI_ShortString = [0; NVAPI_SHORT_STRING_MAX];
    let ret = unsafe { NvAPI_GetInterfaceVersionString(&mut version) };
    if ret != NVAPI_OK {
        println!["Failed to NvAPI_GetInterfaceVersionString: {}", ret];
    }

    let version = String::from_utf8(version.to_vec()).expect("Invalid UTF-8");
    let version = version.trim_matches(char::from(0));
    println!("Interface version: {}", version);

    let mut gpu_handles: [NvPhysicalGpuHandle; NVAPI_MAX_PHYSICAL_GPUS] =
        [NvPhysicalGpuHandle::default(); NVAPI_MAX_PHYSICAL_GPUS];
    let mut gpu_count: NvU32 = 0;
    let ret = unsafe { NvAPI_EnumPhysicalGPUs(&mut gpu_handles, &mut gpu_count) };
    if ret != NVAPI_OK {
        println!["Failed to NvAPI_EnumPhysicalGPUs: {}", ret];
    }
    println!("GPU count: {}", gpu_count);

    for i in 0..gpu_count as usize {
        dbg!(gpu_handles[i]);

        let mut system_type: NV_SYSTEM_TYPE = 0;
        let ret = unsafe { NvAPI_GPU_GetSystemType(gpu_handles[i], &mut system_type) };
        if ret != NVAPI_OK {
            println!["Failed to NvAPI_GPU_GetSystemType: {}", ret];
        }
        let system_type_str = match system_type {
            0 => "unknown",
            1 => "laptop",
            2 => "desktop",
            _ => "invalid",
        };
        println!("System type: {}", system_type_str);

        let mut gpu_type: NV_GPU_TYPE = 0;
        let ret = unsafe { NvAPI_GPU_GetGPUType(gpu_handles[i], &mut gpu_type) };
        if ret != NVAPI_OK {
            println!["Failed to NvAPI_GPU_GetGPUType: {}", ret];
        }
        let gpu_type_str = match gpu_type {
            0 => "unknown",
            1 => "IGPU",
            2 => "DGPU",
            _ => "invalid",
        };
        println!("GPU type: {}", gpu_type_str);

        let mut name: NvAPI_ShortString = [0; NVAPI_SHORT_STRING_MAX];
        let ret = unsafe { NvAPI_GPU_GetFullName(gpu_handles[i], &mut name) };
        if ret != NVAPI_OK {
            println!["Failed to NvAPI_GPU_GetFullName: {}", ret];
        }
        let name = String::from_utf8(name.to_vec()).expect("Invalid UTF-8");
        let name = name.trim_matches(char::from(0));
        println!("Name: {}", name);
    }
}
