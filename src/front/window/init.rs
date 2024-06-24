extern crate winapi;
extern crate lazy_static;

use std::sync::atomic::{AtomicPtr, Ordering};
use winapi::shared::windef::{HBRUSH, HWND};
use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use std::ptr::{null_mut, NonNull};
use std::mem;
use crate::front::window::info;
use crate::front::window::event::massages;
use crate::front::window::utils;
use crate::front::window::register::{save_window_rect, load_window_rect};
use winapi::um::dwmapi::DwmSetWindowAttribute;
use winapi::um::winnt::HRESULT;

// Déclaration d'une variable globale pour stocker le handle de la fenêtre principale
lazy_static::lazy_static! {
    static ref MAIN_WINDOW_HANDLE: AtomicPtr<HWND> = AtomicPtr::new(null_mut());
}

/// Applique la préférence des coins arrondis à une fenêtre
fn apply_window_corner_preference(hwnd: HWND) -> HRESULT {
    let preference: u32 = info::get_dwmwcp_rond();
    unsafe { 
        DwmSetWindowAttribute(
            hwnd, 
            info::get_dwmwa_window_corner_preference(), 
            &preference as *const u32 as *const _, 
            std::mem::size_of::<u32>() as u32
        )
    }
}

/// Définit le handle de la fenêtre principale
pub fn set_main_window_handle(hwnd: HWND) {
    MAIN_WINDOW_HANDLE.store(hwnd as *mut _, Ordering::SeqCst);
}

/// Obtient le handle de la fenêtre principale
pub fn get_main_window_handle() -> Option<HWND> {
    let ptr = MAIN_WINDOW_HANDLE.load(Ordering::SeqCst);
    NonNull::new(ptr).map(|nn| nn.as_ptr() as HWND)
}

/// Crée la fenêtre
pub fn create() {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());
        let class_name = utils::to_wstring("window");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(massages::wnd_proc),
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

        let (width, height, left, top, is_maximized) = load_window_rect();

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            utils::to_wstring("Rust Window").as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            left,
            top,
            width,
            height,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        );

        set_main_window_handle(hwnd);
        apply_window_corner_preference(hwnd);

        if is_maximized {
            ShowWindow(hwnd, SW_MAXIMIZE);
            info::set_is_maximized(true);
        }

        let mut msg: MSG = mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        save_window_rect();
    }
}
