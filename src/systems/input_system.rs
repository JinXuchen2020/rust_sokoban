use std::collections::HashMap;

use ggez::input::keyboard::KeyCode;
use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::{components::{Immovable, Movable, Player, Position}, constants::{MAP_HEIGHT, MAP_WIDTH}, events::{EntityMoved, Event}, resources::{EventQueue, Gameplay, InputQueue}};

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Data
  type SystemData = (
    Write<'a, EventQueue>,
    Write<'a, InputQueue>,
    Write<'a, Gameplay>,
    Entities<'a>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Player>,
    ReadStorage<'a, Movable>,
    ReadStorage<'a, Immovable>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut event_queue, mut input_queue, mut gameplay, entities, mut positions, players, movables, immovables) = data;

    let mut to_move = Vec::new();

    for (position , _player) in (&positions, &players).join() {
      if let Some(key) = input_queue.keys_pressed.pop() {
        let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
        let immov = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

        let (start, end , is_x) = match key {
          KeyCode::Up => (position.y, 0, false),
          KeyCode::Down => (position.y, MAP_HEIGHT, false),
          KeyCode::Left => (position.x, 0, true),
          KeyCode::Right => (position.x, MAP_WIDTH, true),
          _ => continue, 
        };

        let range = if start < end {
          (start..=end).collect::<Vec<_>>()
        } else {
          (end..=start).rev().collect::<Vec<_>>()
        };

        for x_or_y in range {
          let pos = if is_x {
            (x_or_y, position.y)
          } else {
            (position.x, x_or_y)
          };

          // find a movable
          // if it exists, we try to move it and continue
          // if it doesn't exist, we continue and try to find an immovable instead
          match mov.get(&pos) {
            Some(id) => to_move.push((key, id.clone())),
            None => {
                // find an immovable
                // if it exists, we need to stop and not move anything
                // if it doesn't exist, we stop because we found a gap
              match immov.get(&pos) {
                Some(_id) =>{
                  to_move.clear();
                  event_queue.events.push(Event::PlayerHitObstacle);
                },
                None => break,
              }
            }
          }
        }
      }
    }

    if to_move.len() > 0 {
      gameplay.moves_count += 1;
    }

    for (key, id) in to_move {
      let position = positions.get_mut(entities.entity(id));
      
      if let Some(position) = position {
        match key {
          KeyCode::Up => position.y -= 1,
          KeyCode::Down => position.y += 1,
          KeyCode::Left => position.x -= 1,
          KeyCode::Right => position.x += 1,
          _ => (),
        }
      }

      event_queue.events.push(Event::EntityMoved(EntityMoved { id }));
    }
  }
}