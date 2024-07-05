extern crate winapi;
use winapi::shared::windef::{HWND, RECT, POINT};
use winapi::shared::minwindef::{LPARAM, LRESULT};
use winapi::um::winuser::*;
use std::mem;

use crate::front::window::draw::objects::title_bar::winbuttons;
use crate::front::window::info;

/// Fonction pour obtenir la coordonnée x
fn get_x_lparam(lparam: LPARAM) -> i32 {
    (lparam & 0xFFFF) as i16 as i32
}

/// Fonction pour obtenir la coordonnée y
fn get_y_lparam(lparam: LPARAM) -> i32 {
    ((lparam >> 16) & 0xFFFF) as i16 as i32
}

/// Fonction pour gérer les tests de zone de clic
pub unsafe fn handle_nchittest(hwnd: HWND, lparam: LPARAM) -> LRESULT {
    let x = get_x_lparam(lparam);
    let y = get_y_lparam(lparam);

    let mut rect: RECT = mem::zeroed();
    GetWindowRect(hwnd, &mut rect);

    if x >= rect.left && x < rect.left + info::get_border_width() {
        if y >= rect.top && y < rect.top + info::get_border_width() {
            winbuttons::reset_bools();
            return HTTOPLEFT as LRESULT;
        } else if y >= rect.bottom - info::get_border_width() && y < rect.bottom {
            winbuttons::reset_bools();
            return HTBOTTOMLEFT as LRESULT;
        } else {
            winbuttons::reset_bools();
            return HTLEFT as LRESULT;
        }
    } else if x >= rect.right - info::get_border_width() && x < rect.right {
        if y >= rect.top && y < rect.top + info::get_border_width() {
            winbuttons::reset_bools();
            return HTTOPRIGHT as LRESULT;
        } else if y >= rect.bottom - info::get_border_width() && y < rect.bottom {
            winbuttons::reset_bools();
            return HTBOTTOMRIGHT as LRESULT;
        } else {
            winbuttons::reset_bools();
            return HTRIGHT as LRESULT;
        }
    } else if y >= rect.top && y < rect.top + info::get_border_width() {
        winbuttons::reset_bools();
        return HTTOP as LRESULT;
    } else if y >= rect.bottom - info::get_border_width() && y < rect.bottom {
        winbuttons::reset_bools();
        return HTBOTTOM as LRESULT;
    } else if y >= rect.top && y < rect.top + info::get_title_bar_height() {
        if x >= rect.right - info::get_button_width() && x < rect.right {
            winbuttons::reset_bools();
            winbuttons::activ_exit_over();
            return HTCLOSE as LRESULT;
        } else if x >= rect.right - info::get_button_width() * 2 && x < rect.right - info::get_button_width() {
            winbuttons::reset_bools();
            winbuttons::activ_maxim_over();
            return HTMAXBUTTON as LRESULT;
        } else if x >= rect.right - info::get_button_width() * 3 && x < rect.right - info::get_button_width() * 2 {
            winbuttons::reset_bools();
            winbuttons::activ_minim_over();
            return HTMINBUTTON as LRESULT;
        } else {
            winbuttons::reset_bools();
            return HTCAPTION as LRESULT;
        }
    } else {
        winbuttons::reset_bools();
        return HTCLIENT as LRESULT;
    }
}

/// Fonction pour obtenir la position de la souris par rapport à la fenêtre
pub fn get_mouse_position(hwnd: HWND) -> (i32, i32) {
    let mut pt: POINT = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut pt);
        ScreenToClient(hwnd, &mut pt);
    }
    (pt.x, pt.y)
}
