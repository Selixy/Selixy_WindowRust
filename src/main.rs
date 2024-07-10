// src/main.rs

mod windows;

use windows::create::create_window;

fn main() {
    let hwnd = create_window(1, "Ma Fenêtre", 800, 600, 100, 100);
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
