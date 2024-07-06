use crate::front::window::create::WINDOWS;
use crate::front::window::create::Window;
use std::sync::Arc;

pub fn event_loop() {
    loop {
        // Gérer les événements de l'application ici

        // Boucle des fenêtres
        let windows = WINDOWS.lock().unwrap();
        for window in windows.values() {
            handle_window_events(window);
        }
    }
}

fn handle_window_events(_window: &Arc<Window>) {
    // Gérer les événements spécifiques à la fenêtre ici
}
