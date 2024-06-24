extern crate winapi;
extern crate widestring;

use winapi::shared::windef::{HWND, RECT, POINT};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, TRUE};
use winapi::shared::basetsd::LONG_PTR;
use winapi::um::winuser::*;
use std::mem;
use std::ptr::null_mut;
use widestring::U16CStr;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use crate::front::window::draw::winbuttons;
use crate::front::window::utils;
use crate::front::window::register;
use crate::front::window::responsive;
use crate::front::window::event::cliczone::handle_nchittest;
use crate::front::window::draw::draw;
use crate::front::color::darkmode::is_dark_mode_enabled;
use crate::front::color::theme::init_theme;
use crate::front::window::event::event_tick;
use crate::front::window::info;

/// Gère les commandes.
///
/// # Arguments
///
/// * `hwnd` - Handle de la fenêtre.
/// * `wparam` - Paramètre WPARAM de la commande.
///
/// # Retourne
///
/// Le résultat de la commande.
unsafe fn handle_command(hwnd: HWND, wparam: WPARAM) -> LRESULT {
    match utils::loword(wparam as u32) {
        _ => DefWindowProcW(hwnd, WM_COMMAND, wparam, 0),
    }
}

/// Bascule entre maximiser et restaurer la fenêtre.
///
/// # Arguments
///
/// * `hwnd` - Handle de la fenêtre.
pub unsafe fn toggle_maximize_restore(hwnd: HWND) {
    if info::get_is_maximized() {
        info::set_is_maximized(false);
        ShowWindow(hwnd, SW_RESTORE);
    } else {
        info::set_is_maximized(true);
        ShowWindow(hwnd, SW_MAXIMIZE);
    }
}

/// Procédure de la fenêtre (WndProc).
///
/// # Arguments
///
/// * `hwnd` - Handle de la fenêtre.
/// * `msg` - Message envoyé à la fenêtre.
/// * `wparam` - Paramètre WPARAM du message.
/// * `lparam` - Paramètre LPARAM du message.
///
/// # Retourne
///
/// Le résultat du traitement du message.
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
                init_theme(is_dark_mode_enabled());
                SetTimer(hwnd, info::get_timer_id(), info::get_timer_interval(), None);

                // Démarrer un thread en arrière-plan pour des mises à jour périodiques
                let running = Arc::new(AtomicBool::new(true));
                let running_clone = Arc::clone(&running);

                thread::spawn(move || {
                    while running_clone.load(Ordering::SeqCst) {
                        event_tick::event_loop();
                        let sleep_duration = if info::get_is_active() {
                            Duration::from_millis(4)
                        } else {
                            Duration::from_millis(16)
                        };
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
                    info::set_is_minimized(true);
                    0
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            },
            WM_ACTIVATE => {
                if utils::loword(wparam as u32) == WA_INACTIVE {
                    info::set_is_active(false);
                } else {
                    info::set_is_active(true);
                }
                0
            }
            WM_ACTIVATEAPP => {
                if wparam == 0 {
                    info::set_is_active(false);
                } else {
                    info::set_is_active(true);
                }
                0
            }
            WM_TIMER => {
                if wparam == info::get_timer_id() as usize {
                    // Appeler la fonction de mise à jour périodique
                    event_tick::event_loop();
                }
                0
            }
            WM_SETTINGCHANGE => {
                if lparam as isize != 0 {
                    let changed_param = U16CStr::from_ptr_str(lparam as *const u16);
                    if changed_param.to_string_lossy() == "ImmersiveColorSet" {
                        init_theme(is_dark_mode_enabled());
                        InvalidateRect(hwnd, null_mut(), TRUE);
                    }
                }
                0
            }
            WM_NCCALCSIZE => {
                if wparam == 1 {
                    let params = &mut *(lparam as *mut NCCALCSIZE_PARAMS);
                    params.rgrc[0].top -= 1;
                    return 0;
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_SIZE => {
                let width = utils::loword(lparam as u32) as i32;
                let height = utils::hiword(lparam as u32) as i32;
                responsive::set_window_sis(width, height);
                responsive::update_responsive();
                winbuttons::windows_buttons();

                if wparam == SIZE_MINIMIZED {
                    info::set_is_minimized(true);
                } else {
                    info::set_is_minimized(false);
                }

                if wparam == SIZE_MAXIMIZED {
                    info::set_is_maximized(true);
                } else if wparam == SIZE_RESTORED {
                    info::set_is_maximized(false);
                }
                info::set_last_window_rect(hwnd);
                InvalidateRect(hwnd, null_mut(), TRUE);
                0
            }
            WM_MOVE => {
                info::set_last_window_rect(hwnd);
                0
            }
            WM_CLOSE => {
                register::save_window_rect();
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_DESTROY => {
                if !info::get_is_maximized() {
                    info::set_last_window_rect(hwnd);
                }
                KillTimer(hwnd, info::get_timer_id() as usize);

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
                min_max_info.ptMinTrackSize = POINT { x: info::get_min_window_width(), y: info::get_min_window_height() };
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
