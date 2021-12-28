use ggez::{conf, event::{self, KeyCode, KeyMods}, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

use crate::constants::{TILE_WIDTH, MULTIPLIER, TEXT_SIZE, TEXT_PADDING};
use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
    rows: u8,
    cols: u8,
}

impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {
                cols: self.cols,
                rows: self.rows,
            };
            is.run_now(&self.world);
        }
	// Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = RenderingSystem { context, cols: self.cols, rows: self.rows };
            rs.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) -> (u8, u8) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    return load_map(world, MAP.to_string());
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    let (rows, cols) = initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(cols as f32 * TILE_WIDTH * MULTIPLIER, ((rows as f32 * TILE_WIDTH) + (TEXT_SIZE + TEXT_PADDING) * 2.0) * MULTIPLIER),
        )
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world, rows, cols };
    // Run the main event loop
    event::run(context, event_loop, game)
}
