#![allow(non_camel_case_types, non_snake_case)]

pub type NvAPI_Status = i32;
pub type NvU32 = u32;
pub const NVAPI_OK: NvAPI_Status = 0;
pub const NVAPI_SHORT_STRING_MAX: usize = 64;
pub const NVAPI_MAX_PHYSICAL_GPUS: usize = 64;
pub type NvAPI_ShortString = [u8; NVAPI_SHORT_STRING_MAX];
pub type NV_SYSTEM_TYPE = i32;
pub type NV_GPU_TYPE = i32;
pub type NV_MONITOR_CONN_TYPE = i32;

// Display id flags
pub const IS_DYNAMIC: NvU32 = 0x01;
pub const IS_MULTI_STREAM_ROOT_NODE: NvU32 = 0x02;
pub const IS_ACTIVE: NvU32 = 0x04;
pub const IS_CLUSTER: NvU32 = 0x08;
pub const IS_OS_VISIBLE: NvU32 = 0x10;
pub const IS_WFD: NvU32 = 0x20;
pub const IS_CONNECTED: NvU32 = 0x40;
pub const IS_PHYSICALLY_CONNECTED: NvU32 = 0x20000;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct NvPhysicalGpuHandle(*const ::std::os::raw::c_void);

impl Default for NvPhysicalGpuHandle {
    fn default() -> Self {
        NvPhysicalGpuHandle(::std::ptr::null())
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct NV_GPU_DISPLAYIDS {
    pub version: NvU32,
    pub connectorType: NV_MONITOR_CONN_TYPE,
    pub displayId: NvU32,
    pub flags: NvU32,
}

impl Default for NV_GPU_DISPLAYIDS {
    fn default() -> Self {
        NV_GPU_DISPLAYIDS {
            version: 0,
            connectorType: 0,
            displayId: 0,
            flags: 0,
        }
    }
}

pub const NV_GPU_ARCH_INFO_VER1: NvU32 = std::mem::size_of::<NV_GPU_ARCH_INFO>() as NvU32 | 1 << 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct NV_GPU_ARCH_INFO {
    pub version: NvU32,
    pub architecture: NvU32,
    pub implemntation: NvU32,
    pub revision: NvU32,
}

impl Default for NV_GPU_ARCH_INFO {
    fn default() -> Self {
        NV_GPU_ARCH_INFO {
            version: NV_GPU_ARCH_INFO_VER1,
            architecture: 0,
            implemntation: 0,
            revision: 0,
        }
    }
}

pub const NV_GPU_DISPLAYIDS_VER2: NvU32 =
    std::mem::size_of::<NV_GPU_DISPLAYIDS>() as NvU32 | 3 << 16;

#[link(name = "nvapi64", kind = "static")]
extern "stdcall" {
    pub fn NvAPI_Initialize() -> NvAPI_Status;
    pub fn NvAPI_GetInterfaceVersionString(szDesc: *mut NvAPI_ShortString) -> NvAPI_Status;
    pub fn NvAPI_EnumPhysicalGPUs(
        nvGPUHandle: *mut NvPhysicalGpuHandle,
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
    pub fn NvAPI_GPU_GetConnectedDisplayIds(
        hPhysicalGpu: NvPhysicalGpuHandle,
        pDisplayIds: *mut NV_GPU_DISPLAYIDS,
        pDisplayIdCount: *mut NvU32,
        flags: NvU32,
    ) -> NvAPI_Status;
    pub fn NvAPI_GPU_GetArchInfo(
        hPhysicalGpu: NvPhysicalGpuHandle,
        pGpuArchInfo: *mut NV_GPU_ARCH_INFO,
    ) -> NvAPI_Status;
}

fn main() {
    let ret = unsafe { NvAPI_Initialize() };
    if ret != NVAPI_OK {
        println!("Failed to NvAPI_Initialize: {}", ret);
    }

    let mut version: NvAPI_ShortString = [0; NVAPI_SHORT_STRING_MAX];
    let ret = unsafe { NvAPI_GetInterfaceVersionString(&mut version) };
    if ret != NVAPI_OK {
        println!("Failed to NvAPI_GetInterfaceVersionString: {}", ret);
    }

    let version = String::from_utf8(version.to_vec()).expect("Invalid UTF-8");
    let version = version.trim_matches(char::from(0));
    println!("Interface version: {}", version);

    let mut gpu_handles: [NvPhysicalGpuHandle; NVAPI_MAX_PHYSICAL_GPUS] =
        [NvPhysicalGpuHandle::default(); NVAPI_MAX_PHYSICAL_GPUS];
    let mut gpu_count: NvU32 = 0;
    let ret = unsafe { NvAPI_EnumPhysicalGPUs(&mut gpu_handles[0], &mut gpu_count) };
    if ret != NVAPI_OK {
        println!("Failed to NvAPI_EnumPhysicalGPUs: {}", ret);
    }
    println!("GPU count: {}", gpu_count);

    for i in 0..gpu_count as usize {
        dbg!(gpu_handles[i]);

        let mut system_type: NV_SYSTEM_TYPE = 0;
        let ret = unsafe { NvAPI_GPU_GetSystemType(gpu_handles[i], &mut system_type) };
        if ret != NVAPI_OK {
            println!("Failed to NvAPI_GPU_GetSystemType: {}", ret);
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
            println!("Failed to NvAPI_GPU_GetGPUType: {}", ret);
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
            println!("Failed to NvAPI_GPU_GetFullName: {}", ret);
        }
        let name = String::from_utf8(name.to_vec()).expect("Invalid UTF-8");
        let name = name.trim_matches(char::from(0));
        println!("Name: {}", name);

        let mut arch_info = NV_GPU_ARCH_INFO::default();
        let ret = unsafe { NvAPI_GPU_GetArchInfo(gpu_handles[i], &mut arch_info) };
        if ret != NVAPI_OK {
            println!("Failed to NvAPI_GPU_GetArchInfo: {}", ret);
        }
        println!["Received arch: {:?}", arch_info];

        // first, pass in null to get number of display ids
        let mut displayids_count = 0;
        let ret = unsafe {
            NvAPI_GPU_GetConnectedDisplayIds(
                gpu_handles[i],
                ::std::ptr::null_mut(),
                &mut displayids_count,
                0,
            )
        };
        if ret != NVAPI_OK {
            println!("Failed to NvAPI_GPU_GetConnectedDisplayIds: {}", ret);
        }
        println!("Displays: {}", displayids_count);
        if displayids_count > 0 {
            let mut display_ids = NV_GPU_DISPLAYIDS::default();
            display_ids.version = NV_GPU_DISPLAYIDS_VER2;
            let mut display_ids = vec![display_ids; displayids_count as usize];
            let ret = unsafe {
                NvAPI_GPU_GetConnectedDisplayIds(
                    gpu_handles[i],
                    display_ids.as_mut_ptr(),
                    &mut displayids_count,
                    0,
                )
            };
            if ret != NVAPI_OK {
                println!("Failed to NvAPI_GPU_GetConnectedDisplayIds: {}", ret);
            }
            for display_id in display_ids {
                let connector_type = match display_id.connectorType {
                    0 => "unintialized",
                    1 => "vga",
                    2 => "component",
                    3 => "svideo",
                    4 => "hdmi",
                    5 => "dvi",
                    6 => "lvds",
                    7 => "dp",
                    8 => "composite",
                    -1 => "unknown",
                    _ => "",
                };
                println!(
                    "Connector type: {}, id: {}, flags: {}",
                    connector_type, display_id.displayId, display_id.flags
                );
                println!("Dynamic: {}, Multi Stream Root Node: {}, Active: {}, Cluster: {}, Os Visible: {}, Connected: {}, Physically Connected: {}",
                    display_id.flags & IS_DYNAMIC == IS_DYNAMIC,
                    display_id.flags & IS_MULTI_STREAM_ROOT_NODE == IS_MULTI_STREAM_ROOT_NODE,
                    display_id.flags & IS_ACTIVE == IS_ACTIVE,
                    display_id.flags & IS_CLUSTER == IS_CLUSTER,
                    display_id.flags & IS_OS_VISIBLE == IS_OS_VISIBLE,
                    display_id.flags & IS_CONNECTED == IS_CONNECTED,
                    display_id.flags & IS_PHYSICALLY_CONNECTED == IS_PHYSICALLY_CONNECTED);
            }
        }
    }
}
