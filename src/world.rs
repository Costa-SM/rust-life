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
    neighbor_count: Vec<i32>,
    debug: bool,
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
            neighbor_count: vec![0; (row_length * row_length) as usize],
            debug: false,
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

    pub fn switch_debug(&mut self) { 
        self.debug = !self.debug;
        
        for cell in self.cells.iter_mut() {
            cell.switch_debug();
        }
    }

    pub fn change_status(&mut self, x: f32, y: f32) {
        // Width and height of the cells.
        let cell_width = screen_width() / self.row_length;
        let cell_height = screen_height() / self.row_length;

        // Calculate cell position in cell matrix.
        let x_mat = (x / cell_width).floor();
        let y_mat = (y / cell_height).floor();

        if self.debug {
            println!("Mouse click at: {:?}, {:?}", x, y);
            println!("Cell index: {:?}, {:?}", x_mat, y_mat);
        }
        
        // Get the cell's index. Convert to a type that can index the vector.
        let cell_index = (x_mat + y_mat * self.row_length) as usize;

        // Switch the state of the appropriate cell.
        self.cells[cell_index].switch_status();
    }

    pub fn update_world(&mut self) {
        for cell in self.cells.iter() {
            let mut alive_neighbors = 0;

            // Cell's position
            let cell_index = cell.get_index() as usize;
            let cell_x = cell_index % self.row_length as usize;
            let cell_y = cell_index / self.row_length as usize;

            // Index corresponding to the adjacent cells.
            // Allocating them as variables to increase code readability.
            let top_left = cell_index - self.row_length as usize - 1;
            let top_center = cell_index - self.row_length as usize;
            let top_right = cell_index - self.row_length as usize + 1;

            let same_right = cell_index + 1;
            let same_left = cell_index - 1;

            let bottom_left = cell_index + self.row_length as usize - 1;
            let bottom_center = cell_index + self.row_length as usize;
            let bottom_right = cell_index + self.row_length as usize + 1;

            // Edge cases
            if cell_x == 0 {
                if cell_y == 0 {
                    if self.cells[same_right].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_center].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_right].is_alive() { alive_neighbors += 1};
                }

                else if cell_y == (self.row_length - 1f32) as usize {
                    if self.cells[top_center].is_alive() { alive_neighbors += 1};
                    if self.cells[top_right].is_alive() { alive_neighbors += 1};
                    if self.cells[same_right].is_alive() { alive_neighbors += 1};
                }

                else {
                    if self.cells[top_center].is_alive() { alive_neighbors += 1};
                    if self.cells[top_right].is_alive() { alive_neighbors += 1};
                    if self.cells[same_right].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_center].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_right].is_alive() { alive_neighbors += 1};
                }
            }

            else if cell_x == (self.row_length - 1f32) as usize {
                if cell_y == 0 {
                    if self.cells[same_left].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_center].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_left].is_alive() { alive_neighbors += 1};
                }

                else if cell_y == (self.row_length - 1f32) as usize {
                    if self.cells[top_left].is_alive() { alive_neighbors += 1};
                    if self.cells[top_center].is_alive() { alive_neighbors += 1};
                    if self.cells[same_left].is_alive() { alive_neighbors += 1};
                }

                else {
                    if self.cells[top_left].is_alive() { alive_neighbors += 1};
                    if self.cells[top_center].is_alive() { alive_neighbors += 1};
                    if self.cells[same_left].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_left].is_alive() { alive_neighbors += 1};
                    if self.cells[bottom_center].is_alive() { alive_neighbors += 1};
                }
            }

            // Corner cases have already been covered in other conditionals. The program can deal with
            // only the edge cases on this branch.
            else if cell_y == 0 {
                if self.cells[same_left].is_alive() { alive_neighbors += 1};
                if self.cells[same_right].is_alive() { alive_neighbors += 1};
                if self.cells[bottom_left].is_alive() { alive_neighbors += 1};
                if self.cells[bottom_center].is_alive() { alive_neighbors += 1};
                if self.cells[bottom_right].is_alive() { alive_neighbors += 1};
            }

            else if cell_y == (self.row_length - 1f32) as usize {
                if self.cells[top_left].is_alive() { alive_neighbors += 1};
                if self.cells[top_center].is_alive() { alive_neighbors += 1};
                if self.cells[top_right].is_alive() { alive_neighbors += 1};
                if self.cells[same_left].is_alive() { alive_neighbors += 1};
                if self.cells[same_right].is_alive() { alive_neighbors += 1};
            }

            else {
                if self.cells[top_left].is_alive() { alive_neighbors += 1};
                if self.cells[top_center].is_alive() { alive_neighbors += 1};
                if self.cells[top_right].is_alive() { alive_neighbors += 1};
                if self.cells[same_left].is_alive() { alive_neighbors += 1};
                if self.cells[same_right].is_alive() { alive_neighbors += 1};
                if self.cells[bottom_left].is_alive() { alive_neighbors += 1};
                if self.cells[bottom_center].is_alive() { alive_neighbors += 1};
                if self.cells[bottom_right].is_alive() { alive_neighbors += 1};
            }

            self.neighbor_count[cell_index] = alive_neighbors;
        }

        for cell in self.cells.iter_mut() {
            cell.update_cell(self.neighbor_count[cell.get_index() as usize]);
        }
    }

    pub fn run_world(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.run_cell();
        }
    }

    pub fn reset_world(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.kill();
        }
    }

    pub fn draw_world(&self, font: Font) {
        clear_background(BLACK);

        for cell in self.cells.iter() {
            cell.draw_cell(font);
        }
    }
}