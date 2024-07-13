extern crate winapi;

use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, DeleteObject, BitBlt, SRCCOPY, SelectObject};
use winapi::um::winuser::ReleaseDC;
use crate::windows::create::WINDOWS;
use crate::color::theme::get_theme_colors;
use crate::windows::winbuttons::drawing::draw_three_buttons;
use crate::windows::winbuttons::state::*;
use crate::windows::winbuttons::constants::*;
use std::sync::atomic::Ordering;

/// Gère les boutons de la fenêtre.
pub fn windows_buttons() {
    let windows = WINDOWS.lock().unwrap();
    for window in windows.values() {
        let hwnd = window.handle.hwnd;
        let hdc = window.handle.hdc;
        let window_width = window.width as i32;
        let title_bar_height = TITLE_BAR_HEIGHT;
        let button_width = BUTTON_WIDTH;

        if hdc.is_null() {
            return;
        }

        // Créer un DC compatible en mémoire
        let mem_dc = unsafe { CreateCompatibleDC(hdc) };
        if mem_dc.is_null() {
            unsafe { ReleaseDC(hwnd, hdc) };
            return;
        }

        // Créer un bitmap compatible en mémoire
        let mem_bitmap = unsafe { CreateCompatibleBitmap(hdc, window_width, title_bar_height) };
        if mem_bitmap.is_null() {
            unsafe {
                DeleteObject(mem_dc as _);
                ReleaseDC(hwnd, hdc);
            }
            return;
        }

        // Sélectionner le bitmap dans le DC compatible
        unsafe { SelectObject(mem_dc, mem_bitmap as _) };

        // Dessiner les boutons et les icônes sur le DC compatible
        let theme_colors = get_theme_colors();
        draw_three_buttons(mem_dc, &theme_colors, window);

        // Transférer le contenu du DC compatible à l'écran
        unsafe {
            BitBlt(hdc, window_width - button_width * 3, 0, button_width * 3, title_bar_height, mem_dc, window_width - button_width * 3, 0, SRCCOPY);
            DeleteObject(mem_bitmap as _);
            DeleteObject(mem_dc as _);
            ReleaseDC(hwnd, hdc);
        }
    }

    unsafe {
        let mut spid = 0.128;
        for window in windows.values() {
            if window.is_active {
                spid = 0.032;
            }
        }

        if EXIT_OVER.load(Ordering::SeqCst) {
            STOP.store(false, Ordering::SeqCst);
            MAXIM_FACT.store((MAXIM_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            MINIM_FACT.store((MINIM_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            EXIT_FACT.store((EXIT_FACT.load(Ordering::SeqCst) as f32 + spid).min(1.0) as u32, Ordering::SeqCst);
        } else if MAXIM_OVER.load(Ordering::SeqCst) {
            STOP.store(false, Ordering::SeqCst);
            MINIM_FACT.store((MINIM_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            EXIT_FACT.store((EXIT_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            MAXIM_FACT.store((MAXIM_FACT.load(Ordering::SeqCst) as f32 + spid).min(1.0) as u32, Ordering::SeqCst);
        } else if MINIM_OVER.load(Ordering::SeqCst) {
            STOP.store(false, Ordering::SeqCst);
            EXIT_FACT.store((EXIT_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            MAXIM_FACT.store((MAXIM_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            MINIM_FACT.store((MINIM_FACT.load(Ordering::SeqCst) as f32 + spid).min(1.0) as u32, Ordering::SeqCst);
        } else {
            EXIT_FACT.store((EXIT_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            MAXIM_FACT.store((MAXIM_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
            MINIM_FACT.store((MINIM_FACT.load(Ordering::SeqCst) as f32 - spid).max(0.0) as u32, Ordering::SeqCst);
        }

        if !STOP.load(Ordering::SeqCst) {
            if (MINIM_FACT.load(Ordering::SeqCst) + MAXIM_FACT.load(Ordering::SeqCst) + EXIT_FACT.load(Ordering::SeqCst)) <= 0 {
                STOP.store(true, Ordering::SeqCst);
            }
        }
    }
}
