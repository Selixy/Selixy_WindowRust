use crate::str::vectors::Vec2;



// Rectangle
#[derive(Copy, Clone)]

pub struct Rect {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
    pub radius_corner: f64,
}

// Cercle
#[derive(Copy, Clone)]

pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

// Ligne
#[derive(Copy, Clone)]

pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f64,
}