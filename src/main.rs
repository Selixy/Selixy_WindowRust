// src/main.rs

mod windows; // Importation du module `windows`
mod color; // Importation du module `color`

use windows::create::create_window; // Importation de la fonction `create_window`

fn main() {
    // Création de la fenêtre avec des dimensions et une position spécifiques
    let hwnd = create_window("Ma Fenêtre", 800, 600, 100, 100);
    println!("Fenêtre créée avec HWND: {:?}", hwnd);

    // Boucle de messages pour garder la fenêtre ouverte
    unsafe {
        let mut msg: winapi::um::winuser::MSG = std::mem::zeroed();
        while winapi::um::winuser::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            winapi::um::winuser::TranslateMessage(&msg);
            winapi::um::winuser::DispatchMessageW(&msg);
        }
    }
}
