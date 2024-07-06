mod front;
mod str;

use front::window::create::{create_window, handle_events};

fn main() {
    create_window(); // Créer la première fenêtre
    handle_events(); // Gérer les événements tant que des fenêtres sont actives
}
