extern crate winapi; // Importation de la bibliothèque WinAPI
use winapi::shared::windef::{HDC, RECT}; // Importation des types HDC et RECT de WinAPI pour les définitions de contexte de périphérique et de rectangle
use winapi::um::wingdi::{CreateSolidBrush, RGB, DeleteObject}; // Importation des fonctions de dessin graphique de WinAPI
use winapi::um::winuser::FillRect; // Importation de la fonction FillRect de WinAPI
use crate::str::vectors::Rgb; // Importation de la structure Rgb définie dans le projet
use crate::front::color::theme::get_theme_colors; // Importation de la fonction get_theme_colors pour obtenir les couleurs du thème actuel

// Déclaration statique pour la largeur des colonnes
static mut COLUMN_WIDTH: i32 = 220; // Utilisation de i32 pour correspondre aux types des coordonnées RECT

/// Structure représentant un contour de colonne avec ses propriétés
pub struct ColumnOutline {
    pub rect: RECT, // Rectangle définissant la position et la taille du contour de la colonne
    pub color: Rgb, // Couleur du contour de la colonne
}

impl ColumnOutline {
    /// Crée un nouveau contour de colonne avec les dimensions et la couleur spécifiées
    ///
    /// # Arguments
    ///
    /// * `left` - Position gauche du rectangle
    /// * `top` - Position supérieure du rectangle
    /// * `right` - Position droite du rectangle
    /// * `bottom` - Position inférieure du rectangle
    /// * `color` - Couleur du contour de la colonne
    ///
    /// # Retourne
    ///
    /// Une instance de `ColumnOutline`
    pub fn new(left: i32, top: i32, right: i32, bottom: i32, color: Rgb) -> Self {
        ColumnOutline {
            rect: RECT { left, top, right, bottom }, // Initialisation du rectangle avec les dimensions spécifiées
            color, // Initialisation de la couleur
        }
    }

    /// Crée un nouveau contour de colonne à partir des points d'ancrage
    ///
    /// # Arguments
    ///
    /// * `top_left` - Point supérieur gauche
    /// * `bottom_left` - Point inférieur gauche
    ///
    /// # Retourne
    ///
    /// Une instance de `ColumnOutline`
    pub fn create_from_anchors(top_left: (i32, i32), bottom_left: (i32, i32)) -> Self {
        let theme_colors = get_theme_colors(); // Obtenir les couleurs du thème actuel
        ColumnOutline::new(
            top_left.0, top_left.1, top_left.0 + unsafe { COLUMN_WIDTH }, bottom_left.1, theme_colors.background_column
        )
    }

    /// Dessine la colonne dans le contexte de périphérique spécifié
    ///
    /// # Arguments
    ///
    /// * `hdc` - Contexte de périphérique dans lequel dessiner la colonne
    pub fn draw(&self, hdc: HDC) {
        // Crée une brosse solide avec la couleur spécifiée
        let hbrush = unsafe { CreateSolidBrush(RGB(self.color.0, self.color.1, self.color.2)) };
        unsafe {
            // Remplit le rectangle de la colonne avec la brosse créée
            FillRect(hdc, &self.rect, hbrush);

            // Dessine une ligne de séparation verticale sur le bord droit de la colonne
            let theme_colors = get_theme_colors();
            let separator_color = RGB(theme_colors.lignes_separetor.0, theme_colors.lignes_separetor.1, theme_colors.lignes_separetor.2);
            let hbrush_separator = CreateSolidBrush(separator_color);

            let line_rect = RECT {
                left: self.rect.right - 1, // Position de la ligne à 10 pixels du bord droit
                top: self.rect.top,
                right: self.rect.right, // Largeur de la ligne de séparation de 10 pixels
                bottom: self.rect.bottom,
            };

            FillRect(hdc, &line_rect, hbrush_separator);
            DeleteObject(hbrush_separator as _);

            // Supprime l'objet brosse pour éviter les fuites de mémoire
            DeleteObject(hbrush as _);
        }
    }
}
