use crate::{
    audio::AudioStore,
    components::*,
    events::{BoxPlacedOnSpot, EntityMoved, Event},
    resources::EventQueue,
};
use specs::{Entities, Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct EventSystem<'a> {
    pub context: &'a mut ggez::Context,
}

impl<'a> System<'a> for EventSystem<'a> {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
        Entities<'a>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, mut audio_store, entities, boxes, box_spots, positions) = data;

        let mut new_events = Vec::new();

        for event in event_queue.events.drain(..) {
            match event {
                Event::PlayerHitObstacle => {
                    audio_store.play_sound(self.context, &"wall".to_string());
                }
                Event::EntityMoved(EntityMoved { id }) => {
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spots_by_pos: HashMap<(u8, u8), &BoxSpot> =
                            (&box_spots, &positions)
                                .join()
                                .map(|(box_spot, position)| ((position.x, position.y), box_spot))
                                .collect();

                        if let Some(the_position) = positions.get(entities.entity(id)) {
                            if let Some(box_spot) =
                                box_spots_by_pos.get(&(the_position.x, the_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: the_box.color == box_spot.color,
                                }));
                            }
                        }
                    }
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };

                    audio_store.play_sound(self.context, &sound.to_string())
                }
                _ => {}
            }
        }

        event_queue.events.append(&mut new_events);
    }
}
