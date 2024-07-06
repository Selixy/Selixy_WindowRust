use winapi::shared::windef::HWND;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct WindowProperties {
    // Ajouter les propriétés spécifiques de chaque fenêtre ici
}

#[derive(Debug)]
pub struct Window {
    pub id: u32,
    pub hwnd: Arc<Mutex<HWND>>,
    pub properties: WindowProperties,
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
