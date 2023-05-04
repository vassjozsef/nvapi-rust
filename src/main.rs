#![allow(non_camel_case_types)]

pub type NvAPI_Status = i32;
pub const NVAPI_OK: NvAPI_Status = 0;

#[link(name = "nvapi64", kind = "static")]
extern "stdcall" {
    pub fn NvAPI_Initialize() -> NvAPI_Status;
}

fn main() {
    let ret = unsafe { NvAPI_Initialize() };
    if ret != NVAPI_OK {
        println!["Failed to NvAPI: {}", ret];
    }
    println!("NVAPI intialized");
}
