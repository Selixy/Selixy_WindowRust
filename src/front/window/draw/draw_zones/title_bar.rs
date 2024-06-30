extern crate winapi; // Importation de la bibliothèque WinAPI
use winapi::shared::windef::{HDC, RECT}; // Importation des types HDC et RECT de WinAPI pour les définitions de contexte de périphérique et de rectangle
use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, CreateSolidBrush, RGB, DeleteObject, SelectObject, BitBlt, SRCCOPY}; // Importation des fonctions de dessin graphique de WinAPI
use winapi::um::winuser::FillRect; // Importation de la fonction FillRect de WinAPI
use crate::front::window::responsive::get_window_width; // Importation de la fonction get_window_width pour obtenir la largeur de la fenêtre
use crate::front::window::info::{get_button_width, get_title_bar_height}; // Importation des fonctions pour obtenir les dimensions de la barre de titre
use crate::front::color::theme::get_theme_colors; // Importation de la fonction get_theme_colors pour obtenir les couleurs du thème actuel

/// Définir la zone de dessin pour la barre de titre
pub struct TitleBar {
    pub rect: RECT, // Rectangle définissant la position et la taille de la barre de titre
    pub hdc: HDC,   // Contexte de périphérique pour dessiner la barre de titre
}

impl TitleBar {
    /// Crée une nouvelle barre de titre
    ///
    /// # Arguments
    ///
    /// * `hdc` - Contexte de périphérique dans lequel créer la barre de titre
    ///
    /// # Retourne
    ///
    /// Une instance de `TitleBar`
    pub fn new(hdc: HDC) -> Self {
        let width = get_window_width() - get_button_width() * 3; // Calcul de la largeur de la barre de titre
        let height = get_title_bar_height(); // Obtention de la hauteur de la barre de titre
        let rect = RECT { left: 0, top: 0, right: width, bottom: height };

        // Utiliser la couleur de fond de la barre de titre
        let theme_colors = get_theme_colors();
        let hdc = create_compatible_hdc(hdc, width, height, RGB(theme_colors.background.0, theme_colors.background.1, theme_colors.background.2));
        
        TitleBar { rect, hdc }
    }

    /// Transfère la barre de titre au contexte de périphérique principal
    ///
    /// # Arguments
    ///
    /// * `hdc` - Contexte de périphérique principal
    pub fn transfer(&self, hdc: HDC) {
        transfer_zone(hdc, &self.rect, self.hdc);
    }
}

/// Crée un contexte de périphérique compatible
///
/// # Arguments
///
/// * `hdc` - Contexte de périphérique d'origine
/// * `width` - Largeur du contexte de périphérique compatible
/// * `height` - Hauteur du contexte de périphérique compatible
/// * `color` - Couleur de fond du contexte de périphérique compatible
///
/// # Retourne
///
/// Un contexte de périphérique compatible
fn create_compatible_hdc(hdc: HDC, width: i32, height: i32, color: u32) -> HDC {
    unsafe {
        let mem_dc = CreateCompatibleDC(hdc); // Créer un contexte de périphérique compatible en mémoire
        let mem_bitmap = CreateCompatibleBitmap(hdc, width, height); // Créer un bitmap compatible en mémoire
        SelectObject(mem_dc, mem_bitmap as _); // Sélectionner le bitmap dans le contexte de périphérique
        let hbrush = CreateSolidBrush(color); // Créer une brosse solide avec la couleur spécifiée
        FillRect(mem_dc, &RECT { left: 0, top: 0, right: width, bottom: height }, hbrush); // Remplir le rectangle avec la couleur de la brosse
        DeleteObject(hbrush as _); // Supprimer l'objet brosse pour éviter les fuites de mémoire
        mem_dc
    }
}

/// Transfère le contenu d'une zone de dessin au contexte de périphérique principal
///
/// # Arguments
///
/// * `hdc` - Contexte de périphérique principal
/// * `rect` - Rectangle définissant la zone de dessin
/// * `mem_dc` - Contexte de périphérique en mémoire
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
        ); // Transférer le contenu du contexte de périphérique en mémoire à l'écran
        DeleteObject(mem_dc as _); // Supprimer le contexte de périphérique pour éviter les fuites de mémoire
    }
}
