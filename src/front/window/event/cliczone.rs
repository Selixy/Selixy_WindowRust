extern crate winapi;

use winapi::shared::windef::{HWND, RECT}; // Importation des types HWND et RECT
use winapi::shared::minwindef::{LPARAM, LRESULT}; // Importation des types LPARAM et LRESULT
use winapi::um::winuser::*; // Importation des fonctions et constantes utilisateur de winapi
use std::mem; // Importation du module mem pour gérer les structures mémoire

use crate::front::window::draw::winbuttons; // Importation du module buttons
use crate::front::window::info; // Importation du module info


/// Fonction pour obtenir la coordonnée x 
fn get_x_lparam(lparam: LPARAM) -> i32 {
    (lparam & 0xFFFF) as i16 as i32
}

/// Fonction pour obtenir la coordonnée y 
fn get_y_lparam(lparam: LPARAM) -> i32 {
    ((lparam >> 16) & 0xFFFF) as i16 as i32
}

/// Fonction pour gérer les tests de zone de clic
pub unsafe fn handle_nchittest(hwnd: HWND, lparam: LPARAM) -> LRESULT {
    // Obtenir la position x et y du clic
    let x = get_x_lparam(lparam);
    let y = get_y_lparam(lparam);

    // Initialiser une structure RECT vide et obtenir les coordonnées de la fenêtre
    let mut rect: RECT = mem::zeroed();
    GetWindowRect(hwnd, &mut rect);

    // Vérifier la zone de clic par rapport aux bordures de la fenêtre
    // Renvoie la constante correspondante selon la zone cliquée
    if x >= rect.left && x < rect.left + info::get_border_width() {
        if y >= rect.top && y < rect.top + info::get_border_width() {
            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            return HTTOPLEFT as LRESULT; // Clic sur le coin supérieur gauche
        } else if y >= rect.bottom - info::get_border_width() && y < rect.bottom {
            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            return HTBOTTOMLEFT as LRESULT; // Clic sur le coin inférieur gauche
        } else {
            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            return HTLEFT as LRESULT; // Clic sur le bord gauche
        }
    } else if x >= rect.right - info::get_border_width() && x < rect.right {
        if y >= rect.top && y < rect.top + info::get_border_width() {
            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            return HTTOPRIGHT as LRESULT; // Clic sur le coin supérieur droit
        } else if y >= rect.bottom - info::get_border_width() && y < rect.bottom {
            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            return HTBOTTOMRIGHT as LRESULT; // Clic sur le coin inférieur droit
        } else {
            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            return HTRIGHT as LRESULT; // Clic sur le bord droit
        }
    } else if y >= rect.top && y < rect.top + info::get_border_width() {
        winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
        return HTTOP as LRESULT; // Clic sur le bord supérieur
    } else if y >= rect.bottom - info::get_border_width() && y < rect.bottom {
        winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
        return HTBOTTOM as LRESULT; // Clic sur le bord inférieur
    } else if y >= rect.top && y < rect.top + info::get_title_bar_height() {
        // Vérification des clics sur les boutons de la barre de titre
        if x >= rect.right - info::get_button_width() && x < rect.right {

            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            winbuttons::activ_exit_over(); // Activer le survol du bouton de sortie
            return HTCLOSE as LRESULT; // Clic sur le bouton de fermeture
        } else if x >= rect.right - info::get_button_width() * 2 && x < rect.right - info::get_button_width() {

            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            winbuttons::activ_maxim_over(); // Activer le survol du bouton de maximisation/restauration
            return HTMAXBUTTON as LRESULT; // Clic sur le bouton de maximisation/restauration

        } else if x >= rect.right - info::get_button_width() * 3 && x < rect.right - info::get_button_width() * 2 {

            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            winbuttons::activ_minim_over(); // Activer le survol du bouton de minimisation
            return HTMINBUTTON as LRESULT; // Clic sur le bouton de minimisation

        } else {
            winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
            return HTCAPTION as LRESULT; // Clic sur la barre de titre
        }
    } else {
        winbuttons::reset_bools(); // Réinitialiser les booléens pour les boutons
        return HTCLIENT as LRESULT; // Clic sur l'intérieur de la fenêtre
    }
}
