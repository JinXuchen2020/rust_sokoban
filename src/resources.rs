use std::{collections::HashMap, fmt::{self, Display}, time::Duration};

use ggez::{audio, input::keyboard::KeyCode};
use specs::World;

use crate::events::Event;

#[derive(Default)]
pub struct InputQueue {
  pub keys_pressed: Vec<KeyCode>,
}

#[derive(Default)]
pub struct Gameplay {
  pub state: GameplayState,
  pub moves_count: u32
}

#[derive(Default, PartialEq)]
pub enum GameplayState {
  #[default]
  Playing,
  Won
}

impl Display for GameplayState {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(match self {
      GameplayState::Playing => "Playing",
      GameplayState::Won => "Won"
    })?;
    Ok(())
  }
}

#[derive(Default)]
pub struct Time {
  pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {
  pub events: Vec<Event>,
}

#[derive(Default)]
pub struct AudioStore {
  pub sounds: HashMap<String, audio::Source>,
}

pub fn register_resources(world: &mut World) {
  world.insert(InputQueue::default());
  world.insert(Gameplay::default());
  world.insert(Time::default());
  world.insert(EventQueue::default());
  world.insert(AudioStore::default());
}

