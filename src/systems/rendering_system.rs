use glam::Vec2;
use ggez::{Context, graphics::{self, DrawParam, Image, Font, PxScale}};
use specs::{Join, Read, ReadStorage, System};

use crate::constants::{TILE_WIDTH, MULTIPLIER, TEXT_SIZE, TEXT_PADDING};
use crate::components::*;
use crate::resources::*;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
    pub rows: u8,
    pub cols: u8
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
            graphics::DrawParam::new().scale(Vec2::new(MULTIPLIER, MULTIPLIER)).dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }
}

// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (Read<'a, Gameplay>, ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;

        // implementation here
        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH * MULTIPLIER;
            let y = position.y as f32 * TILE_WIDTH * MULTIPLIER;

            // draw
            let draw_params = DrawParam::new().scale(Vec2::new(MULTIPLIER, MULTIPLIER)).dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

	// Render any text
	self.draw_text(&gameplay.state.to_string(), TEXT_PADDING, TILE_WIDTH * self.rows as f32 + TEXT_PADDING);
        self.draw_text(&gameplay.moves_count.to_string(), TEXT_PADDING, TEXT_SIZE + TEXT_PADDING + (TILE_WIDTH * self.rows as f32));

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
