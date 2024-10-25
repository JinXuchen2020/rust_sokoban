use ggez::{event::EventHandler, input::keyboard::KeyInput, Context, GameError, GameResult};
use specs::{System, World, WorldExt};

use components::register_components;
use resources::{register_resources, InputQueue, Time};
use maps::initialize_level;
use systems::event_system::EventSystem;
use systems::game_play_state_system::GameplayStateSystem;
use systems::input_system::InputSystem;
use systems::render_system::RenderSystem;

mod components;
mod resources;
mod systems;
mod entities;
mod maps;
mod constants;
mod events;
pub mod audio;

pub struct Game {
  world: World,
}

impl Game {
  pub fn new(world: World) -> Self {
    Game { world }
  }
}

impl EventHandler<GameError> for Game {
  fn update(&mut self, context: &mut Context) -> GameResult {    
    {      
      let mut input_system = InputSystem {};
      input_system.run(self.world.system_data());
    }

    {
      let mut gss = GameplayStateSystem {};
      gss.run(self.world.system_data());
    }

    {
      let mut time = self.world.write_resource::<Time>();
      time.delta += context.time.delta();
    }

    {
      let mut event_system = EventSystem { context };
      event_system.run(self.world.system_data());
    }

    Ok(())
  }

  fn draw(&mut self, context: &mut Context) -> GameResult {
    {
      let mut rs = RenderSystem::new(context);
      rs.run(self.world.system_data());
    }
    Ok(())
  }

  fn key_down_event(&mut self, _context: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
    let keycode = input.keycode;
    let mut input_queue = self.world.write_resource::<InputQueue>();
    input_queue.keys_pressed.push(keycode.unwrap());       

    Ok(())
  }
}

/*
创建游戏实体
*/
pub fn create_game(world: &mut World) {
  register_components(world);
  register_resources(world);
  
  const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . . . . . W 
    W . P . . RB . W
    W . . . . . . W
    W . . BS . . . W
    W . . . . RS . W
    W W W W W W W W
    ";  
  initialize_level(world, MAP);
}
