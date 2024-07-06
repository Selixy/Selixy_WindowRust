extern crate winapi;
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use winapi::shared::windef::{HBRUSH, HWND};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use winapi::um::dwmapi::DwmSetWindowAttribute;
use winapi::um::winnt::HRESULT;
use std::ptr::null_mut;
use std::mem;
use crate::front::window::info;
use crate::front::window::event::messages;
use crate::front::window::utils;
use crate::front::window::register::{save_window_rect, load_window_rect};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct WindowHandle(HWND);

unsafe impl Send for WindowHandle {}

lazy_static! {
    static ref WINDOWS: Mutex<HashMap<WindowHandle, WindowProperties>> = Mutex::new(HashMap::new());
}

struct WindowProperties {
    // Ajouter les propriétés spécifiques à chaque fenêtre ici
}

/// Applique la préférence des coins arrondis à une fenêtre
fn apply_window_corner_preference(hwnd: HWND) -> HRESULT {
    let preference: u32 = info::get_dwmwcp_rond();
    unsafe {
        DwmSetWindowAttribute(
            hwnd,
            info::get_dwmwa_window_corner_preference(),
            &preference as *const u32 as *const _,
            std::mem::size_of::<u32>() as u32,
        )
    }
}

/// Définit le handle de la fenêtre principale
pub fn set_main_window_handle(hwnd: HWND) {
    let mut windows = WINDOWS.lock().unwrap();
    windows.insert(WindowHandle(hwnd), WindowProperties { /* initialisation des propriétés */ });
}

/// Obtient le handle de la fenêtre principale
pub fn get_main_window_handle() -> Option<HWND> {
    let windows = WINDOWS.lock().unwrap();
    windows.keys().next().map(|wh| wh.0)
}

/// Crée une nouvelle fenêtre et l'ajoute au stockage global
pub fn create_window() -> HWND {
    unsafe {
        let h_instance = GetModuleHandleW(null_mut());
        let class_name = utils::to_wstring("window");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(messages::wnd_proc),
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

        hwnd
    }
}

/// Gère les événements pour toutes les fenêtres
pub fn handle_events() {
    let mut msg: MSG = unsafe { mem::zeroed() };
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } > 0 {
        unsafe {
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
    unsafe {
        save_window_rect();
    }
}

/// Retourne une liste de tous les handles de fenêtre
pub fn get_window_handles() -> Vec<HWND> {
    let windows = WINDOWS.lock().unwrap();
    windows.keys().map(|wh| wh.0).collect()
}
