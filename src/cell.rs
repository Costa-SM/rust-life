// Basic Cell Implementation
use macroquad::prelude::*;

pub enum CellStatus {
    Dead,
    Alive,
}

pub struct Cell {
    cell_index: f32,
    row_length: f32,
    status: CellStatus,
}

impl Cell {
    pub fn new(cell_index: f32, row_length: f32) -> Self {
        Self {
            cell_index,
            row_length,
            status: CellStatus::Dead,
        }
    }

    pub fn is_alive(&self) -> bool {
        match self.status {
            CellStatus::Dead => false,
            CellStatus::Alive => true,
        }
    }

    pub fn draw_cell(&self) {
        // Cell width and height, so that all cells fit in the screen.
        let cell_width = screen_width() / self.row_length;
        let cell_height = screen_height() / self.row_length;
        
        // Calculate cell position in the cell matrix.
        let index_i = (self.cell_index % self.row_length).floor();
        let index_j = (self.cell_index / self.row_length).floor();
        
        // Top-left corner of the cell.
        let draw_x = cell_width * index_i;
        let draw_y = cell_height * index_j;

        if (index_i as i32 % 2 == 0) && (index_j as i32 % 2 == 0) {
            draw_rectangle(draw_x, draw_y, cell_width, cell_height, GREEN);
        }
        else if (index_i as i32 % 2 == 1) && (index_j as i32 % 2 == 0) {
            draw_rectangle(draw_x, draw_y, cell_width, cell_height, YELLOW);
        }
        else if (index_i as i32 % 2 == 0) && (index_j as i32 % 2 == 1) {
            draw_rectangle(draw_x, draw_y, cell_width, cell_height, BLUE);
        }
        else {
            draw_rectangle(draw_x, draw_y, cell_width, cell_height, RED);
        }

        //match self.status {
        //    CellStatus::Alive => (),//draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE),
        //    CellStatus::Dead => (),//draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY)
        //}
    }

    pub fn update_cell(&mut self, alive_neighbors: i32) {
        match alive_neighbors {
            2 => { self.status = CellStatus::Alive }, 
            3 => { self.status = CellStatus::Alive }, 
            _ => { self.status = CellStatus::Dead }, 
        }
    }
}