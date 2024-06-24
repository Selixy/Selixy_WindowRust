extern crate winapi;

use winapi::um::winnt::WCHAR;
use winapi::um::winreg::{RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER};
use winapi::shared::minwindef::DWORD;
use std::ptr::null_mut;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;

/// Chemin de la clé de registre pour les paramètres de personnalisation des thèmes Windows
const KEY_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
/// Nom de la valeur de registre pour le thème des applications
const VALUE_NAME: &str = "AppsUseLightTheme";

/// Vérifie si le mode sombre est activé.
///
/// # Retourne
///
/// `true` si le mode sombre est activé, sinon `false`.
pub fn is_dark_mode_enabled() -> bool {
    // Conversion du chemin de la clé et du nom de la valeur en chaînes Unicode
    let key_path: Vec<WCHAR> = OsString::from(KEY_PATH).encode_wide().chain(Some(0)).collect();
    let value_name: Vec<WCHAR> = OsString::from(VALUE_NAME).encode_wide().chain(Some(0)).collect();
    
    unsafe {
        let mut hkey = null_mut(); // Pointeur pour stocker la clé de registre ouverte
        // Ouverture de la clé de registre
        if RegOpenKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, winapi::um::winnt::KEY_READ, &mut hkey) != 0 {
            return false; // Retourne false si l'ouverture de la clé échoue
        }

        let mut data: DWORD = 0; // Variable pour stocker la donnée de registre
        let mut data_size = std::mem::size_of::<DWORD>() as DWORD; // Taille de la donnée de registre
        // Récupération de la valeur de registre
        if RegQueryValueExW(hkey, value_name.as_ptr(), null_mut(), null_mut(), &mut data as *mut _ as *mut u8, &mut data_size) != 0 {
            return false; // Retourne false si la récupération de la valeur échoue
        }

        data == 0 // Retourne true si le mode sombre est activé, sinon false
    }
}
