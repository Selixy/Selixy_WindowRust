// src/front/window/draw/background.rs

use winapi::shared::windef::{HDC, RECT};
use winapi::um::wingdi::{CreateSolidBrush, RGB, DeleteObject};
use winapi::um::winuser::FillRect;
use crate::front::color::theme::get_theme_colors;
use crate::str::vectors::Rgb;

/// Convertir une structure Rgb en une couleur RGB utilisable par WinAPI
fn color_to_rgb(color: Rgb) -> u32 {
    RGB(color.0, color.1, color.2)
}

/// Dessiner le fond de la fenêtre avec la couleur du thème actuel
pub fn draw_background(hdc: HDC, rect: &RECT) {
    let theme_colors = get_theme_colors(); // Obtenir les couleurs du thème actuel
    let background_color = color_to_rgb(theme_colors.background); // Convertir la couleur de fond en RGB

    unsafe {
        let hbrush = CreateSolidBrush(background_color); // Créer une brosse solide avec la couleur de fond
        FillRect(hdc, rect, hbrush); // Remplir le rectangle de la fenêtre avec la brosse
        DeleteObject(hbrush as _); // Libérer la ressource HBRUSH pour éviter les fuites de mémoire
    }
}
