use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::Mutex;
use lazy_static::lazy_static;
use winapi::shared::windef::{HWND, HBRUSH};
use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT};
use std::os::windows::ffi::OsStrExt;
use crate::windows::{Window, WindowState, WindowHandle};
use winapi::um::dwmapi::DwmSetWindowAttribute;

const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const DWMWCP_ROUND: u32 = 2;

lazy_static! {
    pub static ref WINDOWS: Mutex<HashMap<WindowHandle, Window>> = Mutex::new(HashMap::new());
}

pub fn create_window(title: &str, width: u32, height: u32, x: i32, y: i32) -> HWND {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());
        let class_name = to_wstring("popup");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WS_EX_DLGMODALFRAME | WS_EX_TOPMOST,
            class_name.as_ptr(),
            to_wstring(title).as_ptr(),
            WS_POPUP | WS_VISIBLE,
            x,
            y,
            width as i32,
            height as i32,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );

        apply_window_corner_preference(hwnd);

        let hdc = GetDC(hwnd);

        let handle = WindowHandle { hwnd, hdc };

        WINDOWS.lock().unwrap().insert(handle, Window {
            handle,
            title: title.to_string(),
            width,
            height,
            x,
            y,
            state: WindowState::Restored,
            is_active: true,
        });

        hwnd
    }
}

fn to_wstring(str: &str) -> Vec<u16> {
    std::ffi::OsString::from(str).encode_wide().chain(Some(0).into_iter()).collect()
}

extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            },
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn apply_window_corner_preference(hwnd: HWND) {
    unsafe {
        let preference = DWMWCP_ROUND;
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &preference as *const _ as *const _,
            std::mem::size_of::<u32>() as u32,
        );
    }
}
