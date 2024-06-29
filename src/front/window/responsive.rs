// Importation de la bibliothèque winapi et des modules nécessaires
extern crate winapi;

use crate::front::window::info::{get_info_bar_height, get_title_bar_height};


// Variables locales pour stocker la largeur et la hauteur de la fenêtre
static mut WINDOW_WIDTH: i32 = 600; // Valeur par défaut pour la largeur de la fenêtre
static mut WINDOW_HEIGHT: i32 = 600; // Valeur par défaut pour la hauteur de la fenêtre

// variables locales pour stocker les position des points d'encrage
static mut WORKSPACE_TOP_L: Vec<(i32, i32)> = Vec::new();
static mut WORKSPACE_TOP_R: Vec<(i32, i32)> = Vec::new();
static mut WORKSPACE_BOTTOM_L: Vec<(i32, i32)> = Vec::new();
static mut WORKSPACE_BOTTOM_R: Vec<(i32, i32)> = Vec::new();
static mut WORKSPACE_CENTER: Vec<(i32, i32)> = Vec::new();



fn calculate_workspace_anchors() {
    unsafe {
        let width = WINDOW_WIDTH;
        let height = WINDOW_HEIGHT - get_info_bar_height() - get_title_bar_height();
        let top = get_info_bar_height() + get_title_bar_height();
        let bottom = WINDOW_HEIGHT;
        let left = 0;
        let right = width;
        let center = width / 2;

        WORKSPACE_TOP_L = vec![(left, top)];
        WORKSPACE_TOP_R = vec![(right, top)];
        WORKSPACE_BOTTOM_L = vec![(left, bottom)];
        WORKSPACE_BOTTOM_R = vec![(right, bottom)];
        WORKSPACE_CENTER = vec![(center, top + height / 2)];
    }
}

/// Fonction pour obtenir les positions des points d'ancrage de la zone de travail
pub fn get_workspace_anchors() -> (
    Vec<(i32, i32)>, // Coin supérieur gauche
    Vec<(i32, i32)>, // Coin supérieur droit
    Vec<(i32, i32)>, // Coin inférieur gauche
    Vec<(i32, i32)>, // Coin inférieur droit
    Vec<(i32, i32)>, // Centre
) {
    unsafe {
        (
            WORKSPACE_TOP_L.clone(),
            WORKSPACE_TOP_R.clone(),
            WORKSPACE_BOTTOM_L.clone(),
            WORKSPACE_BOTTOM_R.clone(),
            WORKSPACE_CENTER.clone(),
        )
    }
}

/// Fonction pour mettre à jour les dimensions de la fenêtre
pub unsafe fn set_window_sis(width: i32, height: i32) {
    WINDOW_WIDTH = width;
    WINDOW_HEIGHT = height;
}

pub fn get_window_width() -> i32 {
    unsafe { WINDOW_WIDTH }
}


// Fonction de mise à jour des positions des éléments si redimensionnement
pub unsafe fn update_responsive() {
    calculate_workspace_anchors();

}
