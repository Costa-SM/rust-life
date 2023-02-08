// Camera implementation to pan around the cells.
use macroquad::prelude::*;
pub struct Camera {
    screen: Vec2,
    position: Vec2,
    height: f32,
}

impl Camera {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self{
            screen: Vec2::new(screen_width, screen_height),
            height: 1f32,
            position: Vec2::new(0f32, 0f32),
        }
    }

    pub fn update_screen(&mut self, screen_dimensions: Vec2) {
        self.screen = screen_dimensions;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn update_xy(&mut self, delta: Vec2) {
        self.position += delta;
    }

    pub fn update_z(&mut self, delta: f32) {
        self.height += delta;
    }
}