// src/main.rs

mod windows; // Importation du module `windows`
mod color; // Importation du module `color`

use windows::create::create_window; // Importation de la fonction `create_window`

fn main() {
    create_window(); // Appel de la fonction `create_window`

}
