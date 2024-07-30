// src/windows/mod.rs

pub mod create;

use winapi::shared::windef::{HWND, HDC};

pub struct Fenetre {
    pub id: u16,
    pub hwnd: HWND,
    pub hdc: HDC,
    pub position: [i32; 2],
    pub dimensions: [i32; 2],
}

impl Fenetre {
    pub fn new(id: u16, hwnd: HWND, hdc: HDC, position: [i32; 2], dimensions: [i32; 2]) -> Self {
        Self { id, hwnd, hdc, position, dimensions }
    }

    pub fn set_position(&mut self, position: [i32; 2]) {
        self.position = position;
    }

    pub fn add_position(&mut self, delta: [i32; 2]) {
        self.position[0] += delta[0];
        self.position[1] += delta[1];
    }

    pub fn set_dimensions(&mut self, dimensions: [i32; 2]) {
        self.dimensions = dimensions;
    }

    pub fn add_dimensions(&mut self, delta: [i32; 2]) {
        self.dimensions[0] += delta[0];
        self.dimensions[1] += delta[1];
    }
}

pub struct WindowManager {
    fenetres: Vec<Fenetre>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            fenetres: Vec::new(),
        }
    }

    fn find_smallest_available_id(&self) -> u16 {
        let mut ids: Vec<u16> = self.fenetres.iter().map(|f| f.id).collect();
        ids.sort_unstable();
        for (i, id) in ids.iter().enumerate() {
            if *id != i as u16 {
                return i as u16;
            }
        }
        ids.len() as u16
    }

    pub fn create(&mut self, position: [i32; 2], dimensions: [i32; 2]) {
        let id = self.find_smallest_available_id();
        let (hwnd, hdc) = crate::windows::create::create_window("Window Title", dimensions[0] as u32, dimensions[1] as u32, position[0], position[1]);
        let fenetre = Fenetre::new(id, hwnd, hdc, position, dimensions);
        self.fenetres.push(fenetre);
    }

    pub fn set_position(&mut self, id: u16, position: [i32; 2]) {
        if let Some(fenetre) = self.fenetres.iter_mut().find(|f| f.id == id) {
            fenetre.set_position(position);
        } else {
            println!("Fenetre avec ID {} non trouvée", id);
        }
    }

    pub fn add_position(&mut self, id: u16, delta: [i32; 2]) {
        if let Some(fenetre) = self.fenetres.iter_mut().find(|f| f.id == id) {
            fenetre.add_position(delta);
        } else {
            println!("Fenetre avec ID {} non trouvée", id);
        }
    }

    pub fn set_dimensions(&mut self, id: u16, dimensions: [i32; 2]) {
        if let Some(fenetre) = self.fenetres.iter_mut().find(|f| f.id == id) {
            fenetre.set_dimensions(dimensions);
        } else {
            println!("Fenetre avec ID {} non trouvée", id);
        }
    }

    pub fn add_dimensions(&mut self, id: u16, delta: [i32; 2]) {
        if let Some(fenetre) = self.fenetres.iter_mut().find(|f| f.id == id) {
            fenetre.add_dimensions(delta);
        } else {
            println!("Fenetre avec ID {} non trouvée", id);
        }
    }

    pub fn get(&self, id: u16) -> Option<&Fenetre> {
        self.fenetres.iter().find(|f| f.id == id)
    }
}
