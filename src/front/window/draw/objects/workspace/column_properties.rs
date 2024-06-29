extern crate winapi;
use winapi::shared::windef::{HDC, RECT};
use winapi::um::wingdi::{CreateSolidBrush, RGB, DeleteObject};
use winapi::um::winuser::FillRect;
use crate::str::vectors::Rgb;

/// Structure représentant une colonne avec ses propriétés
pub struct ColumnProperties {
    pub rect: RECT,
    pub color: Rgb,
}

impl ColumnProperties {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32, color: Rgb) -> Self {
        ColumnProperties {
            rect: RECT { left, top, right, bottom },
            color,
        }
    }

    pub fn draw(&self, hdc: HDC) {
        let hbrush = unsafe { CreateSolidBrush(RGB(self.color.0, self.color.1, self.color.2)) };
        unsafe {
            FillRect(hdc, &self.rect, hbrush);
            DeleteObject(hbrush as _);
        }
    }
}
