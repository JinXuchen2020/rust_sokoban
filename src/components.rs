use std::fmt::Display;

use specs::{Component, NullStorage, VecStorage, World, WorldExt};

/*
位置组件
 */
#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
  pub x: u8,
  pub y: u8,
  pub z: u8,
}

/*
渲染组件
 */
#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
  paths: Vec<String>,
}

pub enum RenderableKind {
  Static,
  Animated,
}

impl Renderable {
  pub fn new(paths: Vec<String>) -> Self {
    Self { paths }
  }

  pub fn new_static(path: String) -> Self {
    Self {
      paths: vec![path],
    }
  }

  pub fn kind(&self) -> RenderableKind {
    match self.paths.len() {
        0 => panic!("invalid renderable"),
        1 => RenderableKind::Static,
        _ => RenderableKind::Animated,
    }
  }

  pub fn path(&self, path_index: usize) -> String {
    // If we get asked for a path that is larger than the
    // number of paths we actually have, we simply mod the index
    // with the length to get an index that is in range.
    self.paths[path_index % self.paths.len()].clone()
  }
}

/*
墙组件
 */
#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

/*
玩家组件
*/
#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

/*
箱子组件
*/
#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
  pub color: BoxColor,
}

/*
箱子位置组件
*/
#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
  pub color: BoxColor,
}

#[derive(PartialEq)]
pub enum BoxColor {
  Blue,
  Red
}

impl Display for BoxColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BoxColor::Blue => write!(f, "blue"),
      BoxColor::Red => write!(f, "red"),
    }
  }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

pub fn register_components(world: &mut World) {
  world.register::<Position>();
  world.register::<Renderable>();
  world.register::<Wall>();
  world.register::<Player>();
  world.register::<Box>();
  world.register::<BoxSpot>();
  world.register::<Movable>();
  world.register::<Immovable>();
}