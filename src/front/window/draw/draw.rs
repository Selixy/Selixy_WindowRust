extern crate winapi;
extern crate widestring;

use winapi::shared::windef::{HDC, RECT};
use crate::front::window::draw::background;
use crate::front::window::draw::winbuttons;

// Fonction principale pour dessiner tous les composants de la fenêtre
pub fn draw(hdc: HDC, rect: &RECT) {
    background::draw_background(hdc, rect);
    winbuttons::windows_buttons();
}
