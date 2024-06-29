use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{WINDOWPLACEMENT, GetWindowPlacement, GetWindowRect};
use std::mem;


// Variables globales pour l'état de la fenêtre
static mut IS_MAXIMIZED:     bool = false;
static mut IS_MINIMIZED:     bool = false;
static mut IS_ACTIVE:        bool = true;
static mut LAST_WINDOW_RECT: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };

// Constantes pour les dimentions des elements de la fenêtre
const BORDER_WIDTH:     i32 = 10; // Largeur des bordures de la fenêtre
const BUTTON_WIDTH:     i32 = 47; // Largeur des boutons de la fenêtre

const TITLE_BAR_HEIGHT: i32 = 35; // Hauteur de la barre de titre
const INFO_BAR_HEIGHT:  i32 = 45; // Hauteur de la barre d'information

// Constantes pour l'attribut de coins arrondis
const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const DWMWCP_ROUND:                   u32 = 2;

// Constantes pour les tailles minimales de la fenêtre
const MIN_WINDOW_WIDTH:  i32   = 600;
const MIN_WINDOW_HEIGHT: i32   = 500;
const TIMER_ID:          usize = 1;
const TIMER_INTERVAL:    u32   = 4;


/// Retourne la hauteur de la barre d'information
pub fn get_info_bar_height() -> i32 {
    INFO_BAR_HEIGHT
}

/// Retourne la largeur de la bordure
pub fn get_border_width() -> i32 {
    BORDER_WIDTH
}

/// Retourne la hauteur de la barre de titre
pub fn get_title_bar_height() -> i32 {
    TITLE_BAR_HEIGHT
}

/// Retourne la largeur des boutons
pub fn get_button_width() -> i32 {
    BUTTON_WIDTH
}

/// Retourne la préférence pour les coins arrondis
pub fn get_dwmwa_window_corner_preference() -> u32 {
    DWMWA_WINDOW_CORNER_PREFERENCE
}

/// Retourne la valeur pour les coins arrondis
pub fn get_dwmwcp_rond() -> u32 {
    DWMWCP_ROUND
}

/// Retourne si la fenêtre est maximisée
pub fn get_is_maximized() -> bool {
    unsafe { IS_MAXIMIZED }
}

/// Définit si la fenêtre est maximisée
pub fn set_is_maximized(value: bool) {
    unsafe { IS_MAXIMIZED = value; }
}

/// Définit si la fenêtre est minimisée
pub fn set_is_minimized(value: bool) {
    unsafe { IS_MINIMIZED = value; }
}

/// Retourne si la fenêtre est active
pub fn get_is_active() -> bool {
    unsafe { IS_ACTIVE }
}

/// Définit si la fenêtre est active
pub fn set_is_active(value: bool) {
    unsafe { IS_ACTIVE = value; }
}

/// Retourne le dernier rectangle de la fenêtre
pub fn get_last_window_rect() -> RECT {
    unsafe { LAST_WINDOW_RECT }
}

/// Retourne l'ID du timer
pub fn get_timer_id() -> usize {
    TIMER_ID
}

/// Retourne l'intervalle du timer
pub fn get_timer_interval() -> u32 {
    TIMER_INTERVAL
}

/// Retourne la largeur minimale de la fenêtre
pub fn get_min_window_width() -> i32 {
    MIN_WINDOW_WIDTH
}

/// Retourne la hauteur minimale de la fenêtre
pub fn get_min_window_height() -> i32 {
    MIN_WINDOW_HEIGHT
}

/// Définit le dernier rectangle de la fenêtre
pub fn set_last_window_rect(hwnd: HWND) {
    let mut wp: WINDOWPLACEMENT = unsafe { mem::zeroed() };
    wp.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
    unsafe {
        // Obtenez les informations de placement de la fenêtre
        if GetWindowPlacement(hwnd, &mut wp) != 0 {
            let rect = wp.rcNormalPosition;

            // Assurez-vous que la taille de la fenêtre est au moins la taille minimale
            if rect.right - rect.left >= MIN_WINDOW_WIDTH && rect.bottom - rect.top >= MIN_WINDOW_HEIGHT {
                LAST_WINDOW_RECT = rect;
            }
        } else {
            // Si GetWindowPlacement échoue, utilisez GetWindowRect comme solution de secours
            let mut rect: RECT = mem::zeroed();
            GetWindowRect(hwnd, &mut rect);
            if rect.right - rect.left >= MIN_WINDOW_WIDTH && rect.bottom - rect.top >= MIN_WINDOW_HEIGHT {
                LAST_WINDOW_RECT = rect;
            }
        }
    }
}
