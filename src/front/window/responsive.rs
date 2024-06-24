// Importation de la bibliothèque winapi et des modules nécessaires
extern crate winapi;


// Variables locales pour stocker la largeur et la hauteur de la fenêtre
static mut WINDOW_WIDTH: i32 = 600; // Valeur par défaut pour la largeur de la fenêtre
static mut WINDOW_HEIGHT: i32 = 600; // Valeur par défaut pour la hauteur de la fenêtre

// Fonction pour mettre à jour les dimensions de la fenêtre
pub unsafe fn set_window_sis(width: i32, height: i32) {
    WINDOW_WIDTH = width;
    WINDOW_HEIGHT = height;
}

pub fn get_window_width() -> i32 {
    unsafe { WINDOW_WIDTH }
}




// Fonction de mise à jour des positions des éléments si redimensionnement
pub unsafe fn update_responsive() {

}
