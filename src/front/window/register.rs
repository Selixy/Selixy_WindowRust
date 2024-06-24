extern crate winapi;

use winapi::um::winuser::CW_USEDEFAULT;
use winapi::um::winreg::{RegCreateKeyExW, RegSetValueExW, RegOpenKeyExW, RegGetValueW, HKEY_CURRENT_USER};
use winapi::um::winnt::{KEY_READ, KEY_WRITE, REG_DWORD};
use winapi::shared::minwindef::{HKEY, DWORD};
use std::mem;
use std::ptr::null_mut;
use crate::front::window::info;
use crate::front::window::utils;

// Définition des constantes pour les chemins et noms des clés de registre
const REG_PATH: &str = "Software\\YourAppName";
const REG_WIDTH: &str = "Width";
const REG_HEIGHT: &str = "Height";
const REG_LEFT: &str = "Left";
const REG_TOP: &str = "Top";
const REG_IS_MAXIMIZED: &str = "IsMaximized";
const STATUS_SUCCESS: i32 = 0;

/// Écrire une valeur DWORD dans le registre
unsafe fn write_reg_dword(hkey: HKEY, name: &str, value: DWORD) {
    let name_wstr = utils::to_wstring(name); // Convertir le nom de la clé en wstring
    RegSetValueExW(
        hkey,
        name_wstr.as_ptr(),
        0,
        REG_DWORD,
        &value as *const _ as *const u8,
        mem::size_of::<DWORD>() as u32,
    ); // Enregistrer la valeur DWORD dans le registre
}

/// Lire une valeur DWORD depuis le registre
unsafe fn read_reg_dword(hkey: HKEY, name: &str, default: DWORD) -> DWORD {
    let name_wstr = utils::to_wstring(name); // Convertir le nom de la clé en wstring
    let mut value: DWORD = default; // Valeur par défaut si la lecture échoue
    let mut value_size = mem::size_of::<DWORD>() as u32; // Taille de la valeur

    RegGetValueW(
        hkey,
        null_mut(),
        name_wstr.as_ptr(),
        0x00000002,
        null_mut(),
        &mut value as *mut _ as *mut winapi::ctypes::c_void,
        &mut value_size,
    ); // Lire la valeur DWORD depuis le registre

    value // Retourner la valeur lue
}

/// Sauvegarder la taille et la position de la fenêtre dans le registre
pub unsafe fn save_window_rect() {
    // Déterminer le rectangle à sauvegarder en fonction de l'état de la fenêtre (maximisée ou non)
    let rect_to_save =  info::get_last_window_rect(); // Utiliser les dimensions avant maximisation

    let width = rect_to_save.right - rect_to_save.left; // Calculer la largeur de la fenêtre
    let height = rect_to_save.bottom - rect_to_save.top; // Calculer la hauteur de la fenêtre
    let left = rect_to_save.left; // Obtenir la position gauche
    let top = rect_to_save.top; // Obtenir la position supérieure

    let path = utils::to_wstring(REG_PATH); // Convertir le chemin du registre en wstring
    let mut hkey_result: HKEY = std::mem::zeroed(); // Initialiser une clé de registre vide

    // Créer ou ouvrir la clé de registre pour sauvegarder les valeurs
    if RegCreateKeyExW(
        HKEY_CURRENT_USER,
        path.as_ptr(),
        0,
        null_mut(),
        0,
        KEY_WRITE,
        null_mut(),
        &mut hkey_result,
        null_mut(),
    ) == STATUS_SUCCESS
    {
        // Sauvegarder les valeurs dans le registre
        write_reg_dword(hkey_result, REG_WIDTH, width as DWORD);
        write_reg_dword(hkey_result, REG_HEIGHT, height as DWORD);
        write_reg_dword(hkey_result, REG_LEFT, left as DWORD);
        write_reg_dword(hkey_result, REG_TOP, top as DWORD);
        write_reg_dword(hkey_result, REG_IS_MAXIMIZED, if info::get_is_maximized() { 1 } else { 0 });
    }
}

/// Lire la taille et la position de la fenêtre depuis le registre
pub unsafe fn load_window_rect() -> (i32, i32, i32, i32, bool) {
    let path = utils::to_wstring(REG_PATH); // Convertir le chemin du registre en wstring
    let mut hkey_result: HKEY = std::mem::zeroed(); // Initialiser une clé de registre vide

    // Ouvrir la clé de registre pour lire les valeurs
    if RegOpenKeyExW(HKEY_CURRENT_USER, path.as_ptr(), 0, KEY_READ, &mut hkey_result) == STATUS_SUCCESS {
        // Lire les valeurs depuis le registre
        let width = read_reg_dword(hkey_result, REG_WIDTH, 800);
        let height = read_reg_dword(hkey_result, REG_HEIGHT, 600);
        let left = read_reg_dword(hkey_result, REG_LEFT, CW_USEDEFAULT as DWORD);
        let top = read_reg_dword(hkey_result, REG_TOP, CW_USEDEFAULT as DWORD);
        let is_maximized = read_reg_dword(hkey_result, REG_IS_MAXIMIZED, 0) != 0;

        (width as i32, height as i32, left as i32, top as i32, is_maximized) // Retourner les valeurs lues
    } else {
        (800, 600, CW_USEDEFAULT, CW_USEDEFAULT, false) // Valeurs par défaut si la lecture échoue
    }
}
