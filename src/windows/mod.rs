// src/windows/mod.rs

pub mod create;
pub mod input;
pub mod winbuttons;

use winapi::shared::windef::{HWND, HDC};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct WindowHandle {
    pub hwnd: HWND,
    pub hdc: HDC,
}

unsafe impl Send for WindowHandle {}
unsafe impl Sync for WindowHandle {}

impl PartialEq for WindowHandle {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
    }
}

impl Eq for WindowHandle {}

impl Hash for WindowHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hwnd.hash(state);
    }
}

pub struct Window {
    pub handle: WindowHandle,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub state: WindowState,
    pub is_active: bool,
}

#[derive(Debug, PartialEq)]
pub enum WindowState {
    Maximized,
    Minimized,
    Restored,
}
