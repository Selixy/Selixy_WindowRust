use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Convertit une chaîne de caractères en une wstring (vecteur de u16).
///
/// # Arguments
///
/// * `str` - Une chaîne de caractères à convertir.
///
/// # Retourne
///
/// Un vecteur de u16 représentant la chaîne de caractères encodée.
pub fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str)
        .encode_wide()
        .chain(std::iter::once(0)) // Ajouter un caractère nul à la fin
        .collect()
}

/// Obtient les 16 bits de poids faible d'un u32.
///
/// # Arguments
///
/// * `l` - Le u32 dont on veut obtenir les 16 bits de poids faible.
///
/// # Retourne
///
/// Les 16 bits de poids faible du u32.
pub fn loword(l: u32) -> u16 {
    (l & 0xffff) as u16
}

/// Obtient les 16 bits de poids fort d'un u32.
///
/// # Arguments
///
/// * `l` - Le u32 dont on veut obtenir les 16 bits de poids fort.
///
/// # Retourne
///
/// Les 16 bits de poids fort du u32.
pub fn hiword(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}
