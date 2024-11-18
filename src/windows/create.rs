use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::Mutex;
use lazy_static::lazy_static;
use winapi::shared::windef::{HWND, HDC, HBRUSH};
use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use std::os::windows::ffi::OsStrExt;
use winapi::um::dwmapi::DwmSetWindowAttribute;

const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const DWMWCP_ROUND: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SafeHWND(pub HWND);

#[derive(Clone, Copy)]
pub struct SafeHDC(pub HDC);

unsafe impl Send for SafeHWND {}
unsafe impl Sync for SafeHWND {}

unsafe impl Send for SafeHDC {}
unsafe impl Sync for SafeHDC {}

lazy_static! {
    // Déclaration d'une HashMap protégée par un Mutex pour stocker les fenêtres
    pub static ref WINDOWS: Mutex<HashMap<SafeHWND, SafeHDC>> = Mutex::new(HashMap::new());
}

pub fn create_window(title: &str, width: u32, height: u32, x: i32, y: i32) -> (HWND, HDC) {
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

        let safe_hwnd = SafeHWND(hwnd);
        let safe_hdc = SafeHDC(hdc);

        WINDOWS.lock().unwrap().insert(safe_hwnd, safe_hdc);

        (hwnd, hdc)
    }
}

// Conversion de chaîne en widestring
fn to_wstring(str: &str) -> Vec<u16> {
    std::ffi::OsString::from(str).encode_wide().chain(Some(0).into_iter()).collect()
}

// Dummy window procedure
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w_param: usize, l_param: isize) -> isize {
    match msg {
        WM_DESTROY => {
            let safe_hwnd = SafeHWND(hwnd);
            // Retirer la fenêtre de la HashMap
            WINDOWS.lock().unwrap().remove(&safe_hwnd);
            // Si la dernière fenêtre est fermée, envoyer le message de quitter
            if WINDOWS.lock().unwrap().is_empty() {
                PostQuitMessage(0);
            }
            0
        },
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

// Application des préférences de coin de fenêtre arrondie
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
