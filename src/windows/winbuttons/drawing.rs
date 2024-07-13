extern crate winapi;
extern crate widestring;

use winapi::shared::windef::{HDC, RECT};
use winapi::um::wingdi::*;
use winapi::um::winuser::FillRect;
use widestring::U16CString;
use crate::color::theme::{ThemeColors, Rgb};
use crate::windows::{Window, WindowState};
use crate::windows::winbuttons::constants::*;
use crate::windows::winbuttons::state::*;
use std::sync::atomic::Ordering;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn lerp_color(color1: &Rgb, color2: &Rgb, t: f32) -> Rgb {
    let clamped_t = t.clamp(0.0, 1.0);
    Rgb(
        lerp(color1.0 as f32, color2.0 as f32, clamped_t) as u8,
        lerp(color1.1 as f32, color2.1 as f32, clamped_t) as u8,
        lerp(color1.2 as f32, color2.2 as f32, clamped_t) as u8,
    )
}

fn button_overl(hdc: HDC, color: Rgb, position: (i32, i32)) {
    let (x, y) = position;
    let (r, g, b) = (color.0, color.1, color.2);

    let rect = RECT {
        left: x,
        top: y,
        right: x + BUTTON_WIDTH,
        bottom: y + TITLE_BAR_HEIGHT,
    };

    unsafe {
        let hbrush = CreateSolidBrush(RGB(r, g, b));
        FillRect(hdc, &rect, hbrush);
        DeleteObject(hbrush as _);
    }
}

fn draw_icon(hdc: HDC, factor: f32, position: (i32, i32), icon: char, theme_colors: &ThemeColors) {
    let button_iconselect = if icon != EXIT_ICON {
        &theme_colors.button_iconselect
    } else {
        &Rgb(255, 255, 255)
    };

    let blended_color = lerp_color(&theme_colors.button_icon, button_iconselect, factor);
    let (r, g, b) = (blended_color.0, blended_color.1, blended_color.2);

    let icon_size = 11;

    let icon_position = (
        position.0 + (BUTTON_WIDTH - icon_size) / 2,
        position.1 + (TITLE_BAR_HEIGHT - icon_size) / 2,
    );

    let hfont = unsafe {
        CreateFontW(
            icon_size, 0, 0, 0, 400, 0, 0, 0,
            DEFAULT_CHARSET,
            OUT_DEFAULT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            DEFAULT_QUALITY,
            FF_DONTCARE | DEFAULT_PITCH,
            U16CString::from_str("Segoe MDL2 Assets").unwrap().as_ptr(),
        )
    };

    unsafe {
        SelectObject(hdc, hfont as _);
        SetTextColor(hdc, RGB(r, g, b));
        SetBkMode(hdc, TRANSPARENT as i32);

        let icon_utf16: Vec<u16> = U16CString::from_str(&icon.to_string()).unwrap().into_vec();
        TextOutW(hdc, icon_position.0, icon_position.1, icon_utf16.as_ptr(), icon_utf16.len() as i32);

        DeleteObject(hfont as _);
    }
}

/// Dessine les trois icônes des boutons.
pub fn draw_three_icons(hdc: HDC, theme_colors: &ThemeColors, window: &Window) {
    let minim_position = (window.width as i32 - BUTTON_WIDTH * 3, 0);
    let maxim_position = (window.width as i32 - BUTTON_WIDTH * 2, 0);
    let exit_position = (window.width as i32 - BUTTON_WIDTH, 0);

    let max_rest = if window.state == WindowState::Maximized {
        REST_ICON
    } else {
        MAXIM_ICON
    };

    if window.is_active {
        draw_icon(hdc, EXIT_FACT.load(Ordering::SeqCst) as f32 + 0.8, exit_position, EXIT_ICON, theme_colors);
        draw_icon(hdc, MAXIM_FACT.load(Ordering::SeqCst) as f32 + 0.8, maxim_position, max_rest, theme_colors);
        draw_icon(hdc, MINIM_FACT.load(Ordering::SeqCst) as f32 + 0.8, minim_position, MINIM_ICON, theme_colors);
    } else {
        draw_icon(hdc, EXIT_FACT.load(Ordering::SeqCst) as f32, exit_position, EXIT_ICON, theme_colors);
        draw_icon(hdc, MAXIM_FACT.load(Ordering::SeqCst) as f32, maxim_position, max_rest, theme_colors);
        draw_icon(hdc, MINIM_FACT.load(Ordering::SeqCst) as f32, minim_position, MINIM_ICON, theme_colors);
    }
}

/// Dessine trois boutons en utilisant la fonction button_overl.
pub fn draw_three_buttons(hdc: HDC, theme_colors: &ThemeColors, window: &Window) {
    let background_color = &theme_colors.background;

    let exit_color = &Rgb(232, 17, 35);
    let maxim_color = &Rgb(200, 200, 200);
    let minim_color = &Rgb(200, 200, 200);

    let alpha_minimaxi = 0.4;

    let blended_exit_color = lerp_color(background_color, exit_color, EXIT_FACT.load(Ordering::SeqCst) as f32);
    let blended_maxim_color = lerp_color(background_color, maxim_color, MAXIM_FACT.load(Ordering::SeqCst) as f32 * alpha_minimaxi);
    let blended_minim_color = lerp_color(background_color, minim_color, MINIM_FACT.load(Ordering::SeqCst) as f32 * alpha_minimaxi);

    let minim_position = (window.width as i32 - BUTTON_WIDTH * 3, 0);
    let maxim_position = (window.width as i32 - BUTTON_WIDTH * 2, 0);
    let exit_position = (window.width as i32 - BUTTON_WIDTH, 0);

    button_overl(hdc, blended_exit_color, minim_position);
    button_overl(hdc, blended_maxim_color, maxim_position);
    button_overl(hdc, blended_minim_color, exit_position);

    draw_three_icons(hdc, theme_colors, window);
}
