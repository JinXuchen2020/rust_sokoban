use std::path;
use ggez::{conf, event, GameResult};
use rust_sokoban::{create_game, Game, audio::initialize_sounds};
use specs::{World, WorldExt};
fn main() -> GameResult {
  
  let mut world = World::new();
    // Create a game context and event loop
  let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
      .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
      .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
      .add_resource_path(path::PathBuf::from("./resources"));

  let (mut context, event_loop) = context_builder.build()?;

  create_game(&mut world);
  initialize_sounds(&mut world, &mut context);

  // Create the game state
  let game = Game::new(world);
  // Run the main event loop
  event::run(context, event_loop, game)
}
