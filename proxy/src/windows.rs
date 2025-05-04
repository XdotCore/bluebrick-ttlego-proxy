#![allow(non_snake_case)]

use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use windows::Win32::{
    UI::Input::XboxController::{XINPUT_CAPABILITIES, XINPUT_FLAG, XINPUT_STATE, XINPUT_VIBRATION},
    System::SystemInformation,
};
use std::fs;
use std::sync::OnceLock;

#[derive(WrapperApi)]
struct XInput1_3API {
    XInputGetState: extern "C" fn(wuserindex: u32, pstate: *mut XINPUT_STATE) -> u32,
    XInputSetState: extern "C" fn(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32,
    XInputGetCapabilities: extern "C" fn(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32,
}

fn xinput() -> &'static Container<XInput1_3API> {
    static ONCE: OnceLock<Container<XInput1_3API>> = OnceLock::new();
    ONCE.get_or_init(|| {
        const MAX_PATH_UNICODE: usize = 32767;
        let mut path = [0u16; MAX_PATH_UNICODE];
        let length = unsafe { SystemInformation::GetSystemDirectoryW(Some(&mut path)) };
        if length <= 0 {
            let e = "Could not get system directory to open original for proxy (XInput1_3.dll)";
            let _ = msgbox::create("Error Loading BlueBrick", &e, msgbox::IconType::Error);
            panic!("{e}");
        }
        let path = String::from_utf16_lossy(&path[0..length as usize]);

        let _ = msgbox::create("Folder", &path, msgbox::IconType::Info);

        match unsafe { Container::<XInput1_3API>::load(format!("{path}\\XInput1_3")) } {
            Ok(xinput) => xinput,
            Err(e) => {
                let e = format!("Problem opening original for proxy (XInput1_3.dll):\n{e:?}");
                let _ = msgbox::create("Error Loading BlueBrick", &e, msgbox::IconType::Error);
                panic!("{e}")
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn XInputGetState(wuserindex: u32, pstate: *mut XINPUT_STATE) -> u32 {
    let _ = msgbox::create("err", "XInputGetState", msgbox::IconType::Info);
    xinput().XInputGetState(wuserindex, pstate)
}

#[unsafe(no_mangle)]
pub extern "C" fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32 {
    let _ = msgbox::create("err", "XInputSetState", msgbox::IconType::Info);
    xinput().XInputSetState(dwuserindex, pvibration)
}

#[unsafe(no_mangle)]
pub extern "C" fn XInputGetCapabilities(dwuserindex: u32, dwflags: XINPUT_FLAG, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32 {
    let _ = msgbox::create("err", "XInputGetCapabilities", msgbox::IconType::Info);
    let res = xinput().XInputGetCapabilities(dwuserindex, dwflags, pcapabilities);
    let _ = msgbox::create("err", "XInputGetCapabilities post", msgbox::IconType::Info);
    res
}
