extern crate winapi;
extern crate widestring;

use winapi::shared::windef::{HDC, RECT};
use winapi::um::wingdi::{CreateSolidBrush, RGB, DeleteObject, SelectObject, SetTextColor, SetBkMode, CreateFontW, TextOutW, CreateCompatibleDC, CreateCompatibleBitmap, BitBlt, SRCCOPY};
use winapi::um::winuser::{FillRect, GetDC, ReleaseDC};
use widestring::U16CString;
use crate::front::window::info;
use crate::front::color::theme::{get_theme_colors, ThemeColors};
use crate::str::vectors::Rgb;
use crate::front::window::responsive::get_window_width;
use crate::front::window::init::get_main_window_handle;

// Déclaration des variables statiques pour les facteurs de survol des boutons
static mut EXIT_FACT : f32 = 0.0;
static mut MAXIM_FACT: f32 = 0.0;
static mut MINIM_FACT: f32 = 0.0;

// Déclaration des variables statiques pour l'état de survol des boutons
static mut EXIT_OVER : bool = false;
static mut MAXIM_OVER: bool = false;
static mut MINIM_OVER: bool = false;

// Déclaration de la variable statique pour arrêter le dessin
static mut STOP: bool = false;

// Déclaration des icônes pour les boutons
const EXIT_ICON:  char = '\u{E8BB}'; // Icône pour fermer
const MAXIM_ICON: char = '\u{E922}'; // Icône pour maximiser
const MINIM_ICON: char = '\u{E921}'; // Icône pour minimiser
const REST_ICON:  char = '\u{E923}'; // Icône pour restaurer

/// Réinitialise les états de survol des boutons.
pub fn reset_bools() {
    unsafe {
        EXIT_OVER  = false;
        MAXIM_OVER = false;
        MINIM_OVER = false;
    }
}

/// Active l'état de survol du bouton de sortie.
pub fn activ_exit_over() {
    unsafe {
        EXIT_OVER = true;
    }
}

/// Active l'état de survol du bouton de maximisation.
pub fn activ_maxim_over() {
    unsafe {
        MAXIM_OVER = true;
    }
}

/// Active l'état de survol du bouton de minimisation.
pub fn activ_minim_over() {
    unsafe {
        MINIM_OVER = true;
    }
}

/// Effectue l'interpolation linéaire entre deux valeurs avec clamping.
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Effectue l'interpolation linéaire entre deux couleurs avec clamping du facteur entre 0 et 1.
fn lerp_color(color1: Rgb, color2: Rgb, t: f32) -> Rgb {
    let clamped_t = t.clamp(0.0, 1.0);
    Rgb(
        lerp(color1.0 as f32, color2.0 as f32, clamped_t) as u8,
        lerp(color1.1 as f32, color2.1 as f32, clamped_t) as u8,
        lerp(color1.2 as f32, color2.2 as f32, clamped_t) as u8,
    )
}

/// Dessine un carré de couleur spécifiée à la position donnée.
fn button_overl(hdc: HDC, color: Rgb, position: (i32, i32)) {
    let button_width = info::get_button_width();
    let title_bar_height = info::get_title_bar_height();

    let (x, y) = position;
    let (r, g, b) = (color.0, color.1, color.2);

    let rect = RECT {
        left: x,
        top: y,
        right: x + button_width,
        bottom: y + title_bar_height,
    };

    unsafe {
        let hbrush = CreateSolidBrush(RGB(r, g, b));
        FillRect(hdc, &rect, hbrush);
        DeleteObject(hbrush as _);
    }
}

/// Dessine une icône sur un bouton.
fn draw_icon(hdc: HDC, factor: f32, position: (i32, i32), icon: char, theme_colors: &ThemeColors) {
    let button_iconselect = if icon != EXIT_ICON { 
        theme_colors.button_iconselect
    } else {
        Rgb( 255, 255, 255)
    };

    let blended_color = lerp_color(theme_colors.button_icon, button_iconselect, factor);
    let (r, g, b) = (blended_color.0, blended_color.1, blended_color.2);

    let button_width = info::get_button_width();
    let title_bar_height = info::get_title_bar_height();

    let icon_size = 11;

    let icon_position = (
        position.0 + (button_width - icon_size) / 2,
        position.1 + (title_bar_height - icon_size) / 2,
    );

    let hfont = unsafe {
        CreateFontW(
            icon_size, 0, 0, 0, 400, 0, 0, 0,
            winapi::um::wingdi::DEFAULT_CHARSET,
            winapi::um::wingdi::OUT_DEFAULT_PRECIS,
            winapi::um::wingdi::CLIP_DEFAULT_PRECIS,
            winapi::um::wingdi::DEFAULT_QUALITY,
            winapi::um::wingdi::FF_DONTCARE | winapi::um::wingdi::DEFAULT_PITCH,
            U16CString::from_str("Segoe MDL2 Assets").unwrap().as_ptr(),
        )
    };

    unsafe {
        SelectObject(hdc, hfont as _);
        SetTextColor(hdc, RGB(r, g, b));
        SetBkMode(hdc, winapi::um::wingdi::TRANSPARENT as i32);

        let icon_utf16: Vec<u16> = U16CString::from_str(&icon.to_string()).unwrap().into_vec();
        TextOutW(hdc, icon_position.0, icon_position.1, icon_utf16.as_ptr(), icon_utf16.len() as i32);

        DeleteObject(hfont as _);
    }
}

/// Dessine les trois icônes des boutons.
fn draw_three_icones(hdc: HDC, theme_colors: &ThemeColors) {
    unsafe {
        let button_width = info::get_button_width();
        let window_width = get_window_width();

        let minim_position = (window_width - button_width * 3, 0);
        let maxim_position = (window_width - button_width * 2, 0);
        let exit_position = (window_width - button_width, 0);

        let max_rest = if info::get_is_maximized() {
            REST_ICON
        } else {
            MAXIM_ICON
        };

        draw_icon(hdc, EXIT_FACT, exit_position, EXIT_ICON, theme_colors);
        draw_icon(hdc, MAXIM_FACT, maxim_position, max_rest, theme_colors);
        draw_icon(hdc, MINIM_FACT, minim_position, MINIM_ICON, theme_colors);
    }
}

/// Dessine trois boutons en utilisant la fonction button_overl.
fn draw_three_buttons(hdc: HDC, theme_colors: &ThemeColors) {
    unsafe {
        let background_color = theme_colors.background;

        let exit_color  = Rgb( 232,  17,  35);
        let maxim_color = Rgb( 200, 200, 200);
        let minim_color = Rgb( 200, 200, 200);

        let alpha_minimaxi = 0.4;

        let blended_exit_color  = lerp_color(background_color, exit_color , EXIT_FACT);
        let blended_maxim_color = lerp_color(background_color, maxim_color, MAXIM_FACT * alpha_minimaxi);
        let blended_minim_color = lerp_color(background_color, minim_color, MINIM_FACT * alpha_minimaxi);

        let button_width = info::get_button_width();
        let window_width = get_window_width();

        let minim_position = (window_width - button_width * 3, 0);
        let maxim_position = (window_width - button_width * 2, 0);
        let exit_position  = (window_width - button_width    , 0);

        button_overl(hdc, blended_exit_color , exit_position);
        button_overl(hdc, blended_maxim_color, maxim_position);
        button_overl(hdc, blended_minim_color, minim_position);

        draw_three_icones(hdc, theme_colors);
    }
}

/// Gère les boutons de la fenêtre.
pub fn windows_buttons() {
    if let Some(hwnd) = get_main_window_handle() {
        let hdc = unsafe { GetDC(hwnd) };
        if hdc.is_null() {
            return;
        }

        // Créer un DC compatible en mémoire
        let mem_dc = unsafe { CreateCompatibleDC(hdc) };
        if mem_dc.is_null() {
            unsafe { ReleaseDC(hwnd, hdc) };
            return;
        }

        // Créer un bitmap compatible en mémoire
        let window_width = get_window_width();
        let title_bar_height = info::get_title_bar_height();
        let button_width = info::get_button_width();
        let mem_bitmap = unsafe { CreateCompatibleBitmap(hdc, window_width, title_bar_height) };
        if mem_bitmap.is_null() {
            unsafe {
                DeleteObject(mem_dc as _);
                ReleaseDC(hwnd, hdc);
            }
            return;
        }

        // Sélectionner le bitmap dans le DC compatible
        unsafe { SelectObject(mem_dc, mem_bitmap as _) };

        // Dessiner les boutons et les icônes sur le DC compatible
        let theme_colors = get_theme_colors();
        draw_three_buttons(mem_dc, &theme_colors);

        // Transférer le contenu du DC compatible à l'écran
        unsafe {
            BitBlt(hdc, window_width - button_width * 3, 0, button_width * 3, title_bar_height, mem_dc, window_width - button_width * 3, 0, SRCCOPY);
            DeleteObject(mem_bitmap as _);
            DeleteObject(mem_dc as _);
            ReleaseDC(hwnd, hdc);
        }
    }

    unsafe {
        let mut spid = 0.128;
        if info::get_is_active() {
            spid = 0.032;
        }

        if EXIT_OVER {
            STOP = false;
            MAXIM_FACT = (MAXIM_FACT - spid).max(0.0);
            MINIM_FACT = (MINIM_FACT - spid).max(0.0);
            EXIT_FACT  = (EXIT_FACT  + spid).min(1.0);
        } else if MAXIM_OVER {
            STOP = false;
            MINIM_FACT = (MINIM_FACT - spid).max(0.0);
            EXIT_FACT  = (EXIT_FACT  - spid).max(0.0);
            MAXIM_FACT = (MAXIM_FACT + spid).min(1.0);
        } else if MINIM_OVER {
            STOP = false;
            EXIT_FACT  = (EXIT_FACT  - spid).max(0.0);
            MAXIM_FACT = (MAXIM_FACT - spid).max(0.0);
            MINIM_FACT = (MINIM_FACT + spid).min(1.0);
        } else {
            EXIT_FACT  = (EXIT_FACT  - spid).max(0.0);
            MAXIM_FACT = (MAXIM_FACT - spid).max(0.0);
            MINIM_FACT = (MINIM_FACT - spid).max(0.0);
        }

        if !STOP {
            if MINIM_FACT + MAXIM_FACT + EXIT_FACT <= 0.0 {
                STOP = true;
            }
        }
    }
}
