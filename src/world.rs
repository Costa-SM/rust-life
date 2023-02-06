// Implementation of the world the cells live in.
use macroquad::{prelude::*, miniquad::FrontFaceOrder};

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
        // Width and height of the cells.
        let cell_width = screen_width() / self.row_length;
        let cell_height = screen_height() / self.row_length;

        // Calculate cell position in cell matrix.
        let x_mat = (x / cell_width).floor();
        let y_mat = (y / cell_height).floor();

        println!("Cell index: {:?}, {:?}", x_mat, y_mat);

        // Get the cell's index. Convert to a type that can index the vector.
        let cell_index = (x_mat + y_mat * self.row_length) as usize;

        // Switch the state of the appropriate cell.
        self.cells[cell_index].switch_status();
    }

    pub fn update_world(&mut self) {
        for cell in self.cells.iter() {
            let current_index = cell.get_index() as i32;
            let mut alive_neighbors = 0;

            // // Check neighbors
            // // Border cases
            // // Cell is at the top of the board.
            // if (cell.get_index() / self.row_length).floor() as i32 == 0 {

            // }
            // // Cell is at the bottom of the board.
            // else if (cell.get_index() / self.row_length).floor() as i32 == (self.row_length - 1f32) as i32 {

            // }

            // // Cell is not in a border
            // if self.cells[(current_index - self.row_length as i32 - 1) as usize].is_alive() { alive_neighbors +=1; };
            // if self.cells[(current_index - self.row_length as i32 + 1) as usize].is_alive() { alive_neighbors +=1; };
            // if self.cells[(current_index - 1) as usize].is_alive() { alive_neighbors +=1; };
            // if self.cells[(current_index + 1) as usize].is_alive() { alive_neighbors +=1; };
            // if self.cells[(current_index + self.row_length as i32 - 1) as usize].is_alive() { alive_neighbors +=1; };
            // if self.cells[(current_index + self.row_length as i32 + 1) as usize].is_alive() { alive_neighbors +=1; };

        }
    }

    pub fn draw_world(&self, font: Font) {
        clear_background(BLACK);

        for cell in self.cells.iter() {
            cell.draw_cell(font);
        }
    }
}