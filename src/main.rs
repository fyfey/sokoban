use crate::audio::initialize_sounds;
use ggez::{
    conf,
    event::{self, KeyCode, KeyMods},
    timer, Context, GameResult,
};
use specs::{Dispatcher, DispatcherBuilder, RunNow, World, WorldExt};
use std::path;

mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod map;
mod resources;
mod systems;

use crate::components::*;
use crate::constants::{MULTIPLIER, TEXT_PADDING, TEXT_SIZE, TILE_WIDTH};
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
    rows: u8,
    cols: u8,
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        println!(
            "[update] ticks: {}\tfps: {}\tdelta: {:?}",
            timer::ticks(ctx),
            timer::fps(ctx),
            timer::delta(ctx)
        );

        self.dispatcher.dispatch(&mut self.world);
        {
            let mut es = EventSystem { context: ctx };
            es.run_now(&self.world);
        }
        self.world.maintain();
        let mut time = self.world.write_resource::<Time>();
        time.delta += timer::delta(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Render game entities
        println!(
            "[draw] ticks: {}\tfps: {}\tdelta: {:?}",
            timer::ticks(ctx),
            timer::fps(ctx),
            timer::delta(ctx)
        );
        {
            let mut rs = RenderingSystem {
                context: ctx,
                cols: self.cols,
                rows: self.rows,
            };
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
    W . . . BR BB . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . SR SB . . W
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
        .window_mode(conf::WindowMode::default().dimensions(
            cols as f32 * TILE_WIDTH * MULTIPLIER,
            ((rows as f32 * TILE_WIDTH) + (TEXT_SIZE + TEXT_PADDING) * 2.0) * MULTIPLIER,
        ))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;

    let dispatcher = DispatcherBuilder::new()
        .with(
            InputSystem {
                cols: cols,
                rows: rows,
            },
            "input",
            &[],
        )
        .with(GameplayStateSystem {}, "gameplay", &[])
        .build();
    initialize_sounds(&mut world, &mut context);
    // Create the game state
    let game = Game {
        world,
        dispatcher,
        rows,
        cols,
    };
    // Run the main event loop
    event::run(context, event_loop, game)
}
