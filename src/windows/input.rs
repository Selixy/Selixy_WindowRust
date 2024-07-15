extern crate winapi;

use winapi::shared::windef::{HWND, RECT};
use winapi::shared::minwindef::{LPARAM, LRESULT};
use winapi::um::winuser::*;
use std::mem;

use crate::windows::winbuttons::constants::{BUTTON_WIDTH, TITLE_BAR_HEIGHT};
use crate::windows::winbuttons::state::{reset_bools, activate_button_hover};

const BORDER_WIDTH: i32 = 10;

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

    if x >= rect.left && x < rect.left + BORDER_WIDTH {
        if y >= rect.top && y < rect.top + BORDER_WIDTH {
            reset_bools();
            return HTTOPLEFT as LRESULT;
        } else if y >= rect.bottom - BORDER_WIDTH && y < rect.bottom {
            reset_bools();
            return HTBOTTOMLEFT as LRESULT;
        } else {
            reset_bools();
            return HTLEFT as LRESULT;
        }
    } else if x >= rect.right - BORDER_WIDTH && x < rect.right {
        if y >= rect.top && y < rect.top + BORDER_WIDTH {
            reset_bools();
            return HTTOPRIGHT as LRESULT;
        } else if y >= rect.bottom - BORDER_WIDTH && y < rect.bottom {
            reset_bools();
            return HTBOTTOMRIGHT as LRESULT;
        } else {
            reset_bools();
            return HTRIGHT as LRESULT;
        }
    } else if y >= rect.top && y < rect.top + BORDER_WIDTH {
        reset_bools();
        return HTTOP as LRESULT;
    } else if y >= rect.bottom - BORDER_WIDTH && y < rect.bottom {
        reset_bools();
        return HTBOTTOM as LRESULT;
    } else if y >= rect.top && y < rect.top + TITLE_BAR_HEIGHT {
        if x >= rect.right - BUTTON_WIDTH && x < rect.right {
            reset_bools();
            activate_button_hover("exit");
            return HTCLOSE as LRESULT;
        } else if x >= rect.right - BUTTON_WIDTH * 2 && x < rect.right - BUTTON_WIDTH {
            reset_bools();
            activate_button_hover("maximize");
            return HTMAXBUTTON as LRESULT;
        } else if x >= rect.right - BUTTON_WIDTH * 3 && x < rect.right - BUTTON_WIDTH * 2 {
            reset_bools();
            activate_button_hover("minimize");
            return HTMINBUTTON as LRESULT;
        } else {
            reset_bools();
            return HTCAPTION as LRESULT;
        }
    } else {
        reset_bools();
        return HTCLIENT as LRESULT;
    }
}
