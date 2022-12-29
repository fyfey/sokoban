use ggez::graphics::spritebatch::SpriteBatch;
use ggez::{
    graphics::{self, DrawParam, Font, Image, PxScale},
    Context,
};
use glam::Vec2;
use itertools::Itertools;
use specs::{Join, Read, ReadStorage, System};
use std::collections::HashMap;
use std::time::Duration;

use crate::components::*;
use crate::constants::{MULTIPLIER, TEXT_PADDING, TEXT_SIZE, TILE_WIDTH};
use crate::resources::*;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
    pub rows: u8,
    pub cols: u8,
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;

        // implementation here
        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image_path = self.get_image(renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH * MULTIPLIER;
            let y = position.y as f32 * TILE_WIDTH * MULTIPLIER;
            let z = position.z;

            // draw
            let draw_params = DrawParam::new()
                .scale(Vec2::new(MULTIPLIER, MULTIPLIER))
                .dest(Vec2::new(x, y));
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_params);
        }

        // Render any text
        self.draw_text(
            &gameplay.state.to_string(),
            TEXT_PADDING,
            TILE_WIDTH * self.rows as f32 + TEXT_PADDING,
        );
        self.draw_text(
            &gameplay.moves_count.to_string(),
            TEXT_PADDING,
            TEXT_SIZE + TEXT_PADDING + (TILE_WIDTH * self.rows as f32),
        );

        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(a.0, b.0))
        {
            for (image_path, draw_params) in group {
                let image = Image::new(self.context, image_path).expect("expected image");
                let mut sprite_batch = SpriteBatch::new(image);

                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("expected render");
            }
        }
        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let arcade_font = Font::new(self.context, "/ARCADE_N.TTF");
        let mut text = graphics::Text::new(text_string);
        text.set_font(arcade_font.unwrap(), PxScale::from(TEXT_SIZE));
        let destination = Vec2::new(x * MULTIPLIER, y * MULTIPLIER);
        let color = Some(graphics::Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 0.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new()
                .scale(Vec2::new(MULTIPLIER, MULTIPLIER))
                .dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }
    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
        let path_index = match renderable.kind() {
            RenderableKind::Static => {
                // We only have one image, so we just return that
                0
            }
            RenderableKind::Animated => {
                // If we have multiple, we want to select the right one based on the delta time.
                // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
                // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
                // we technically are on the next iteration of the loop (or on 0), but we will let
                // the renderable handle this logic of wrapping frames.
                ((delta.as_millis() % 1000) / 250) as usize
            }
        };

        renderable.path(path_index)
    }
}
