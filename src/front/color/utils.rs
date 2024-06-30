/// Fonction d'interpolation linéaire des couleurs
///
/// # Arguments
///
/// * `start_color` - Couleur de départ sous forme de tuple (r, g, b)
/// * `end_color` - Couleur de fin sous forme de tuple (r, g, b)
/// * `t` - Facteur d'interpolation (0.0 <= t <= 1.0)
///
/// # Retourne
///
/// Une couleur interpolée sous forme de tuple (r, g, b)
pub fn lerp_color(start_color: (u8, u8, u8), end_color: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    let t = f64::max(0.0, f64::min(1.0, t));
    let r = (start_color.0 as f64 * (1.0 - t) + end_color.0 as f64 * t) as u8;
    let g = (start_color.1 as f64 * (1.0 - t) + end_color.1 as f64 * t) as u8;
    let b = (start_color.2 as f64 * (1.0 - t) + end_color.2 as f64 * t) as u8;
    (r, g, b)
}
