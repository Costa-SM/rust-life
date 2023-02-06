// Macroquad for graphics
use macroquad::{prelude::*, window};
use world::WorldState;

// Local implementations
mod cell;
mod world;

// Auxiliary things
const ROW_LENGTH: f32 = 64f32; // Number of cells in a row. World will be composed of ROW_LENGTH^2 cells.

// Main Function
#[macroquad::main("Game of Life")]
async fn main() {
    let mut world = world::CellWorld::new(ROW_LENGTH);
    let font = load_ttf_font("res/Heebo-VariableFont_wght.ttf").await.unwrap();

    loop {
        // Draw world before everything else, so that we can draw on top of it, if necessary.
        world.draw_world(font);

        if is_key_pressed(KeyCode::Space) {
            world.switch_state()
        }

        match world.get_state() {
            WorldState::Running => {
                world.update_world();

                if is_mouse_button_released(MouseButton::Left) {
                    let mp = mouse_position();
                    world.change_status(mp.0, mp.1);
                
                    println!("Mouse click at: {:?}, {:?}", mp.0, mp.1);
                }
            }

            WorldState::Stopped => {
                // "Press Space to Resume"
                let pause_text = screen_width().to_string();
                let pause_dims = measure_text(&pause_text, Some(font), 30u16, 1.0f32);
                
                draw_rectangle(screen_width() * 0.5f32 - pause_dims.width * 0.5f32 - 5f32,
                    50f32 - pause_dims.height - 5f32,
                    pause_dims.width + 10f32,
                    pause_dims.height + 10f32,
                    BLACK);

                draw_text_ex(
                    &pause_text, 
                    screen_width() * 0.5f32 - pause_dims.width * 0.5f32, 
                    50f32, 
                    TextParams{font, font_size: 30u16, color: WHITE, ..Default::default()}
                );
            }
        }
        
        next_frame().await
    }
}
