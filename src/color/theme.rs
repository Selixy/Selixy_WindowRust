// src/color/theme.rs

pub struct Rgb(pub u8, pub u8, pub u8);


// Structure pour stocker les couleurs du thème
pub struct ThemeColors {
    pub background:          Rgb, // Couleur de fond
    pub lignes_separetor:    Rgb, // Couleur de séparation
    pub background_info_bar: Rgb, // Couleur de fond 
    pub background_column:   Rgb, // Couleur de fond des colonnes
    pub button_icon:         Rgb, // Couleur de texte des boutons
    pub button_iconselect:   Rgb, // Couleur de texte des boutons sélectionnés
}

// Variable statique pour stocker le thème actuel
static mut CURRENT_THEME: Option<ThemeColors> = None;

// Constantes pour les couleurs du thème sombre et clair
pub const DARK_THEME: ThemeColors = ThemeColors {
    background:              Rgb(  18,  18,  18),
    lignes_separetor:        Rgb(  18,  18,  18),
    background_info_bar:     Rgb(  32,  32,  32),
    background_column:       Rgb(  32,  32,  32),
    button_icon:             Rgb( 128, 128, 128),
    button_iconselect:       Rgb( 255, 255, 255),
};

pub const LIGHT_THEME: ThemeColors = ThemeColors {
    background:              Rgb( 255, 255, 255),
    lignes_separetor:        Rgb( 200, 200, 200),
    background_info_bar:     Rgb( 240, 240, 240),
    background_column:       Rgb( 240, 240, 240),
    button_icon:             Rgb( 150, 150, 150),
    button_iconselect:       Rgb(  32,  32,  32),
};

/// Initialise le thème basé sur le mode sombre ou clair.
///
/// # Arguments
///
/// * `is_dark_mode` - Un booléen indiquant si le mode sombre est activé.
pub fn init_theme(is_dark_mode: bool) {
    unsafe {
        // Assigner les couleurs du thème en fonction du mode sombre ou clair
        CURRENT_THEME = Some(if is_dark_mode {
            DARK_THEME
        } else {
            LIGHT_THEME
        });
    }
}

/// Obtient les couleurs du thème actuel.
///
/// # Retourne
///
/// Une référence immuable vers les couleurs du thème actuel.
///
/// # Panique
///
/// La fonction panique si le thème n'a pas été initialisé.
pub fn get_theme_colors() -> &'static ThemeColors {
    unsafe {
        // Renvoie une référence aux couleurs du thème actuel, ou génère une erreur si le thème n'est pas initialisé
        CURRENT_THEME.as_ref().expect("Theme not initialized")
    }
}