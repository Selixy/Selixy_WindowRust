extern crate winapi;
use winapi::shared::windef::{HDC, RECT};
use winapi::um::wingdi::{CreateSolidBrush, RGB, DeleteObject};
use winapi::um::winuser::FillRect;
use crate::front::window::draw::objects::workspace::column_properties::ColumnProperties;
use crate::front::window::draw::objects::workspace::column_outline::ColumnOutline;
use crate::front::window::responsive::get_workspace_anchors;
use crate::front::color::theme::get_theme_colors;

/// Structure représentant l'espace de travail
pub struct Workspace {
    pub columns: Vec<ColumnProperties>,
    pub outlines: Vec<ColumnOutline>,
}

impl Workspace {
    /// Crée un nouvel espace de travail avec des colonnes et des contours initiaux
    pub fn new(_hdc: HDC) -> Self {
        // Calcule les points d'ancrage de la zone de travail
        let (top_left, top_right, bottom_left, bottom_right, _center) = get_workspace_anchors();

        let columns = vec![
            ColumnProperties::create_from_anchors(top_right[0], bottom_right[0]), // Exemple de colonne
        ];

        let outlines = vec![
            ColumnOutline::create_from_anchors(top_left[0], bottom_left[0]), // Exemple de contour
        ];

        Workspace { columns, outlines }
    }

    /// Transfère le contenu de l'espace de travail au contexte de périphérique principal
    pub fn transfer(&self, hdc: HDC) {
        // Peindre le fond de la zone de travail
        self.paint_background(hdc);

        // Dessiner les colonnes
        for column in &self.columns {
            column.draw(hdc);
        }

        // Dessiner les contours de colonne
        for outline in &self.outlines {
            outline.draw(hdc);
        }
    }

    /// Peindre le fond de la zone de travail
    fn paint_background(&self, hdc: HDC) {
        let theme_colors = get_theme_colors();
        let background_color = theme_colors.background;
        let hbrush = unsafe { CreateSolidBrush(RGB(background_color.0, background_color.1, background_color.2)) };

        // Calcule les points d'ancrage de la zone de travail
        let (_, top_right, _, bottom_right, _center) = get_workspace_anchors();
        let left = 0;
        let top = top_right[0].1;
        let right = bottom_right[0].0;
        let bottom = bottom_right[0].1;

        let rect = RECT { left, top, right, bottom };

        unsafe {
            FillRect(hdc, &rect, hbrush);
            DeleteObject(hbrush as _);
        }
    }
}
