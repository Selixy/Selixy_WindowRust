// src/main.rs

mod windows;
mod color;

use windows::WindowManager;
use std::ptr::null_mut;

fn main() {
    let mut window_manager = WindowManager::new();

    // Crée deux fenêtres avec des valeurs différentes
    window_manager.create([50, 50], [800, 600]);
    window_manager.create([300, 500], [800, 600]);


    // Lecture des propriétés de la fenêtre avec ID 1
    if let Some(fenetre) = window_manager.get(0) {
        println!(
            "Fenetre ID 0: hwnd = {:?}, hdc = {:?}, position = {:?}, dimensions = {:?}",
            fenetre.hwnd, fenetre.hdc, fenetre.position, fenetre.dimensions
        );
    } else {
        println!("Fenetre avec ID 0 non trouvée");
    }

    // Run the message loop to keep the windows responsive
    unsafe {
        let mut msg = std::mem::zeroed();
        while winapi::um::winuser::GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            winapi::um::winuser::TranslateMessage(&msg);
            winapi::um::winuser::DispatchMessageW(&msg);
        }
    }
}