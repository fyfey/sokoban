use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::{
    components::{Box, BoxColor, BoxSpot, Position},
    resources::{Gameplay, GameplayState},
};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    // Data
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, positions, boxes, box_spots) = data;

        // get all boxes indexed by position
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();
        let mut boxes_by_color: HashMap<BoxColor, HashMap<(u8, u8), &Box>> = HashMap::new();
        for (pos, _box) in boxes_by_position.iter() {
            boxes_by_color
                .entry(_box.color)
                .or_default()
                .insert((pos.0, pos.1), _box);
        }

        // loop through all box spots and check if there is a corresponding
        // box at that position
        gameplay_state.state = GameplayState::Playing;
        for (_box_spot, position) in (&box_spots, &positions).join() {
            if !boxes_by_color.contains_key(&_box_spot.color) {
                return;
            }
            if boxes_by_color[&_box_spot.color].contains_key(&(position.x, position.y)) {
                continue;
            }
            return;
        }

        // If we made it this far, then all box spots have boxes on them, and the
        // game has been won
        gameplay_state.state = GameplayState::Won;
    }
}
