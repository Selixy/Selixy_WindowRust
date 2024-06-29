extern crate winapi;
extern crate widestring;

use winapi::shared::windef::{HDC, RECT};
use crate::front::window::draw::winbuttons;
use crate::front::window::draw::draw_zones::{title_bar::TitleBar, info_bar::InfoBar, workspace::Workspace};

/// Fonction principale de dessin
pub fn draw(hdc: HDC, rect: &RECT) {
    // Gérer les boutons de la fenêtre
    winbuttons::windows_buttons();

    // Gérer les zones de dessin
    let title_bar = TitleBar::new(hdc);
    let info_bar = InfoBar::new(hdc);
    let workspace = Workspace::new(hdc);

    title_bar.transfer(hdc);
    info_bar.transfer(hdc);
    workspace.transfer(hdc);
}
