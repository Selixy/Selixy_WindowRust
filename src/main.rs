mod front;
mod str;

use front::window::create::{create_window, handle_events};

fn main() {
    create_window("Main Window", None); // Créer la première fenêtre
    crate::front::window::create::handle_events(); // Gérer les événements
}