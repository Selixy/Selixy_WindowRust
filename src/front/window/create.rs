extern crate lazy_static;
extern crate winapi;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::shared::windef::{HBRUSH, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use crate::front::window::utils::to_wstring;
use crate::front::window::register::save_window_rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SafeHWND(HWND);

pub struct WindowProperties {
    // Définissez les propriétés de la fenêtre ici
}

unsafe impl Send for SafeHWND {}
unsafe impl Sync for SafeHWND {}

lazy_static! {
    static ref WINDOWS: Mutex<HashMap<SafeHWND, WindowProperties>> = Mutex::new(HashMap::new());
}

pub fn create_window(title: &str, parent: Option<HWND>) -> HWND {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());
        let class_name = to_wstring("window");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(crate::front::window::event::messages::wnd_proc),
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
            0,
            class_name.as_ptr(),
            to_wstring(title).as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            parent.unwrap_or(null_mut()),
            null_mut(),
            h_instance,
            null_mut(),
        );

        WINDOWS.lock().unwrap().insert(SafeHWND(hwnd), WindowProperties {
            // Initialisez les propriétés de la fenêtre ici
        });

        hwnd
    }
}

pub fn get_main_window_handle() -> Option<HWND> {
    let windows = WINDOWS.lock().unwrap();
    windows.keys().next().map(|safe_hwnd| safe_hwnd.0)
}

pub fn get_window_handles() -> Vec<HWND> {
    let windows = WINDOWS.lock().unwrap();
    windows.keys().map(|safe_hwnd| safe_hwnd.0).collect()
}

pub fn handle_events() {
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        save_window_rect();  // Utilisez save_window_rect de manière appropriée
    }
}
