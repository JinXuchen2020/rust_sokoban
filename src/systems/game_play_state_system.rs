use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::{components::{Box, BoxSpot, Position}, events::Event, resources::{EventQueue, Gameplay, GameplayState}};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
  type SystemData = (
    Write<'a, EventQueue>,
    Write<'a, Gameplay>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Box>,
    ReadStorage<'a, BoxSpot>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut event_queue, mut game_play, postions, boxes, boxspots) = data;

    let box_positions: HashMap<(u8,u8), &Box> = (&postions, &boxes).join().map(|t| ((t.0.x, t.0.y), t.1)).collect();

    for (position, boxspot) in (&postions, &boxspots).join() {
      let match_box = box_positions.get(&(position.x, position.y));
      
      if let Some(bx) = match_box {
        if bx.color == boxspot.color {
          continue;
        } else {
          return;
        }
      } else {
        game_play.state = GameplayState::Playing;
        return;
      }
    }

    game_play.state = GameplayState::Won;
    event_queue.events.push(Event::GameOver);
  }
}