// src/windows/mod.rs

pub mod create;

#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub is_maximized: bool,
    pub is_minimized: bool,
}

