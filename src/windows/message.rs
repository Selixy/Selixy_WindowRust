extern crate winapi;

use std::mem;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use winapi::shared::windef::{HWND, RECT, POINT};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, TRUE};
use winapi::um::winuser::*;
use winapi::um::dwmapi::DwmSetWindowAttribute;
use winapi::shared::basetsd::LONG_PTR;
use crate::windows::{WindowState};
use crate::windows::create::WINDOWS;
use crate::windows::input::handle_nchittest;
use widestring::U16CStr;
use crate::windows::{utils, event_tick, draw, responsive, register};
use crate::windows::winbuttons::logic;

// Définit la préférence de coin de la fenêtre
const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const DWMWCP_ROUND: u32 = 2;

// Applique la préférence de coin de fenêtre arrondie
unsafe fn apply_window_corner_preference(hwnd: HWND) {
    let preference: u32 = DWMWCP_ROUND;
    DwmSetWindowAttribute(
        hwnd,
        DWMWA_WINDOW_CORNER_PREFERENCE,
        &preference as *const u32 as *const _,
        mem::size_of::<u32>() as u32,
    );
}

unsafe fn handle_command(hwnd: HWND, wparam: WPARAM) -> LRESULT {
    match utils::loword(wparam as u32) {
        _ => DefWindowProcW(hwnd, WM_COMMAND, wparam, 0),
    }
}

pub unsafe fn toggle_maximize_restore(hwnd: HWND) {
    let mut windows = WINDOWS.lock().unwrap();
    if let Some((_, window)) = windows.iter_mut().find(|(_, win)| win.handle.hwnd == hwnd) {
        if window.state == WindowState::Maximized {
            window.state = WindowState::Restored;
            ShowWindow(hwnd, SW_RESTORE);
        } else {
            window.state = WindowState::Maximized;
            ShowWindow(hwnd, SW_MAXIMIZE);
        }
    }
}

pub extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                let mut rect: RECT = mem::zeroed();
                GetWindowRect(hwnd, &mut rect);
                SetWindowPos(
                    hwnd,
                    null_mut(),
                    rect.left,
                    rect.top,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                    SWP_FRAMECHANGED | SWP_NOZORDER | SWP_NOACTIVATE,
                );

                SetTimer(hwnd, 1, 16, None);

                apply_window_corner_preference(hwnd);

                let running = Arc::new(AtomicBool::new(true));
                let running_clone = Arc::clone(&running);

                thread::spawn(move || {
                    while running_clone.load(Ordering::SeqCst) {
                        event_tick::event_loop();
                        let sleep_duration = Duration::from_millis(16);
                        thread::sleep(sleep_duration);
                    }
                });

                SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(Box::new(running)) as LONG_PTR);

                0
            }
            WM_COMMAND => handle_command(hwnd, wparam),
            WM_PAINT => {
                let mut ps: PAINTSTRUCT = mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                let rect = ps.rcPaint;
                draw::draw(hdc, &rect);
                EndPaint(hwnd, &ps);
                0
            }
            WM_NCHITTEST => handle_nchittest(hwnd, lparam),
            WM_NCLBUTTONDOWN => match wparam as isize {
                HTCLOSE => {
                    PostQuitMessage(0);
                    0
                }
                HTMAXBUTTON => {
                    toggle_maximize_restore(hwnd);
                    0
                }
                HTMINBUTTON => {
                    ShowWindow(hwnd, SW_MINIMIZE);
                    let mut windows = WINDOWS.lock().unwrap();
                    if let Some((_, window)) = windows.iter_mut().find(|(_, win)| win.handle.hwnd == hwnd) {
                        window.state = WindowState::Minimized;
                    }
                    0
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            },
            WM_ACTIVATE => {
                let mut windows = WINDOWS.lock().unwrap();
                if let Some((_, window)) = windows.iter_mut().find(|(_, win)| win.handle.hwnd == hwnd) {
                    window.is_active = utils::loword(wparam as u32) != WA_INACTIVE;
                }
                0
            }
            WM_ACTIVATEAPP => {
                let mut windows = WINDOWS.lock().unwrap();
                if let Some((_, window)) = windows.iter_mut().find(|(_, win)| win.handle.hwnd == hwnd) {
                    window.is_active = wparam != 0;
                }
                0
            }
            WM_TIMER => {
                if wparam == 1 {
                    event_tick::event_loop();
                }
                0
            }
            WM_SETTINGCHANGE => {
                if lparam as isize != 0 {
                    let changed_param = U16CStr::from_ptr_str(lparam as *const u16);
                    if changed_param.to_string_lossy() == "ImmersiveColorSet" {
                        InvalidateRect(hwnd, null_mut(), TRUE);
                    }
                }
                0
            }
            WM_NCCALCSIZE => {
                if wparam == 1 {
                    let params = &mut *(lparam as *mut NCCALCSIZE_PARAMS);
                    params.rgrc[0].top;
                    params.rgrc[0].left;
                    params.rgrc[0].right;
                    params.rgrc[0].bottom;
                    return 0;
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_SIZE => {
                let width = utils::loword(lparam as u32) as i32;
                let height = utils::hiword(lparam as u32) as i32;
                responsive::set_window_sis(width, height);
                responsive::update_responsive();
                logic::windows_buttons();

                let mut windows = WINDOWS.lock().unwrap();
                if let Some((_, window)) = windows.iter_mut().find(|(_, win)| win.handle.hwnd == hwnd) {
                    if wparam == SIZE_MINIMIZED {
                        window.state = WindowState::Minimized;
                    } else {
                        window.state = WindowState::Restored;
                    }

                    if wparam == SIZE_MAXIMIZED {
                        window.state = WindowState::Maximized;
                    } else if wparam == SIZE_RESTORED {
                        window.state = WindowState::Restored;
                    }
                }
                0
            }
            WM_MOVE => {
                let mut windows = WINDOWS.lock().unwrap();
                if let Some((_, window)) = windows.iter_mut().find(|(_, win)| win.handle.hwnd == hwnd) {
                    // Mettre à jour les coordonnées de la fenêtre si nécessaire
                }
                0
            }
            WM_CLOSE => {
                register::save_window_rect();
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_DESTROY => {
                let mut windows = WINDOWS.lock().unwrap();
                if let Some((_, window)) = windows.iter().find(|(_, win)| win.handle.hwnd == hwnd) {
                    if window.state != WindowState::Maximized {
                        // Mettre à jour le dernier rectangle de la fenêtre si nécessaire
                    }
                }
                KillTimer(hwnd, 1);

                let running_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Arc<AtomicBool>;
                if !running_ptr.is_null() {
                    (*running_ptr).store(false, Ordering::SeqCst);
                    drop(Box::from_raw(running_ptr));
                }

                PostQuitMessage(0);
                0
            }
            WM_GETMINMAXINFO => {
                let min_max_info = &mut *(lparam as *mut MINMAXINFO);
                min_max_info.ptMinTrackSize = POINT { x: 400, y: 400 };
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
