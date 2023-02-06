// Basic Cell Implementation
use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum CellStatus {
    Dead,
    Alive,
}

pub struct Cell {
    cell_index: f32,
    row_length: f32,
    status: CellStatus,
    alive_neighbors: i32,
}

impl Cell {
    pub fn new(cell_index: f32, row_length: f32) -> Self {
        Self {
            cell_index,
            row_length,
            status: CellStatus::Dead,
            alive_neighbors: 0,
        }
    }

    pub fn switch_status(&mut self) {
        match self.status {
            CellStatus::Dead => self.status = CellStatus::Alive,
            CellStatus::Alive => self.status = CellStatus::Dead,
        }
    }

    pub fn get_index(&self) -> f32 {
        self.cell_index
    }

    pub fn is_alive(&self) -> bool {
        match self.status {
            CellStatus::Dead => false,
            CellStatus::Alive => true,
        }
    }

    pub fn draw_cell(&self, font: Font) {
        // Cell width and height, so that all cells fit in the screen.
        let cell_width = screen_width() / self.row_length;
        let cell_height = screen_height() / self.row_length;
        
        // Calculate cell position in the cell matrix.
        let index_i = (self.cell_index % self.row_length).floor();
        let index_j = (self.cell_index / self.row_length).floor();
        
        // Top-left corner of the cell.
        let draw_x = cell_width * index_i;
        let draw_y = cell_height * index_j;

        // Draw the cell
        let cell_margin = 1f32;
        draw_rectangle(draw_x, draw_y, cell_width, cell_height, BLACK);

        if self.status == CellStatus::Alive {
            draw_rectangle(draw_x + cell_margin, draw_y + cell_margin, cell_width - 2f32 * cell_margin, cell_height- 2f32 * cell_margin, WHITE);
        }

        // Draw a debug alive neighbor counter.
        let text_counter = self.alive_neighbors.to_string();
        let font_size = 10u16;
        let text_dims = measure_text(&text_counter, Some(font), font_size, 1.0f32);
        
        draw_text_ex(
            &text_counter, 
            draw_x + cell_width * 0.5f32 - text_dims.width * 0.5f32,
            draw_y + cell_height * 0.5f32 + text_dims.height * 0.5f32,
            TextParams{font, font_size: font_size, color: RED, ..Default::default()}
        );
    }

    pub fn update_cell(&mut self, alive_neighbors: i32) {
        self.alive_neighbors = alive_neighbors;
        
        match alive_neighbors {
            2 => { self.status = CellStatus::Alive }, 
            3 => { self.status = CellStatus::Alive }, 
            _ => { self.status = CellStatus::Dead }, 
        }
    }
}