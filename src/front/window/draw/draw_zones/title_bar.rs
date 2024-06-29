extern crate winapi;
use winapi::shared::windef::{HDC, RECT};
use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, CreateSolidBrush, RGB, DeleteObject, SelectObject, BitBlt, SRCCOPY};
use winapi::um::winuser::FillRect;
use crate::front::window::responsive::get_window_width;
use crate::front::window::info::{get_button_width, get_title_bar_height};
use crate::front::color::theme::get_theme_colors;

// Définir la zone de dessin pour la barre de titre
pub struct TitleBar {
    pub rect: RECT,
    pub hdc: HDC,
}

impl TitleBar {
    pub fn new(hdc: HDC) -> Self {
        let width = get_window_width() - get_button_width() * 3;
        let height = get_title_bar_height();
        let rect = RECT { left: 0, top: 0, right: width, bottom: height };

        // Utiliser la couleur de fond de la barre de titre
        let theme_colors = get_theme_colors();
        let hdc = create_compatible_hdc(hdc, width, height, RGB(theme_colors.background.0, theme_colors.background.1, theme_colors.background.2));
        
        TitleBar { rect, hdc }
    }

    pub fn transfer(&self, hdc: HDC) {
        transfer_zone(hdc, &self.rect, self.hdc);
    }
}

fn create_compatible_hdc(hdc: HDC, width: i32, height: i32, color: u32) -> HDC {
    unsafe {
        let mem_dc = CreateCompatibleDC(hdc);
        let mem_bitmap = CreateCompatibleBitmap(hdc, width, height);
        SelectObject(mem_dc, mem_bitmap as _);
        let hbrush = CreateSolidBrush(color);
        FillRect(mem_dc, &RECT { left: 0, top: 0, right: width, bottom: height }, hbrush);
        DeleteObject(hbrush as _);
        mem_dc
    }
}

fn transfer_zone(hdc: HDC, rect: &RECT, mem_dc: HDC) {
    unsafe {
        BitBlt(
            hdc,
            rect.left,
            rect.top,
            rect.right - rect.left,
            rect.bottom - rect.top,
            mem_dc,
            0,
            0,
            SRCCOPY,
        );
        DeleteObject(mem_dc as _);
    }
}
