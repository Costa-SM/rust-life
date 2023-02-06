// Implementation of the world the cells live in.
use macroquad::prelude::*;

pub enum WorldState {
    Running,
    Stopped,
}

pub struct CellWorld {
    row_length: f32,
    state: WorldState,
    cells: Vec<crate::cell::Cell>,
}

impl CellWorld {
    pub fn new(row_length: f32) -> Self {
        let mut world_vector = Vec::new();

        for cell_index in 0..(row_length*row_length) as i32 {
            world_vector.push(crate::cell::Cell::new(cell_index as f32, row_length));
        }

        println!("World Length: {:?}", world_vector.len().to_string());

        Self {
            row_length,
            state: WorldState::Stopped,
            cells: world_vector,
        }
    }
    
    pub fn get_state(&self) -> WorldState{
        match self.state {
            WorldState::Running => WorldState::Stopped,
            WorldState::Stopped => WorldState::Running,
        }
    }

    pub fn switch_state(&mut self) {
        match self.state {
            WorldState::Running => self.state = WorldState::Stopped,
            WorldState::Stopped => self.state = WorldState::Running,
        }
    }


    pub fn change_status(&mut self, x: f32, y: f32) {

    }

    pub fn update_world(&mut self) {

    }

    pub fn draw_world(&self) {
        clear_background(BLACK);

        for cell in self.cells.iter() {
            cell.draw_cell();
        }
    }
}