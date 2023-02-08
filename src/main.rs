use std::default;

// Macroquad for graphics
use macroquad::prelude::*;
use world::WorldState;

// Local implementations
mod cell;
mod world;
mod camera;

// Auxiliary things.
const ROW_LENGTH: f32 = 128f32; // Number of cells in a row. World will be composed of ROW_LENGTH^2 cells.

fn write_text(x: f32, y: f32, text: &str, font: Font) {
    let text_dims = measure_text(text, Some(font), 30u16, 1.0f32);
    
    draw_rectangle(
        x - text_dims.width * 0.5f32,
        y - text_dims.height * 0.5f32,
        text_dims.width + 10f32,
        text_dims.height + 10f32,
        BLACK);

    draw_text_ex(
        &text, 
        x - text_dims.width * 0.5f32 + 5f32,
        y + text_dims.height * 0.5f32, 
        TextParams{font, font_size: 30u16, color: WHITE, ..Default::default()}
    );
}
// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_resizable: false,
        window_height: 900,
        window_width: 900,
        ..Default::default()
    }
}


// Main Function
#[macroquad::main(window_conf)]
async fn main() {
    let mut world = world::CellWorld::new(ROW_LENGTH);
    let font = load_ttf_font("res/Heebo-VariableFont_wght.ttf").await.unwrap();

    loop {
        // Draw world before everything else, so that we can draw on top of it, if necessary.
        world.draw_world(font);

        // Handle input
        if is_key_pressed(KeyCode::Space) {
            world.switch_state();
        }

        if is_key_pressed(KeyCode::R) {
            world.reset_world();
        }

        if is_key_pressed(KeyCode::D) {
            world.switch_debug();
        }

        if is_mouse_button_released(MouseButton::Left) {
            let mp = mouse_position();
            world.change_status(mp.0, mp.1);
        }
            
        // Update the world.
        world.update_world();
        match world.get_state() {
            WorldState::Running => {
                world.run_world();
            }

            WorldState::Stopped => {
                write_text(screen_width() * 0.5f32, 40f32, "Press Space to Resume", font);
                write_text(screen_width() * 0.5f32, 75f32, "Press R to Reset", font);
            }
        }
        
        next_frame().await
    }
}
