use specs::{Builder, World, WorldExt};

use crate::components::{Box, BoxColor, BoxSpot, Immovable, Movable, Player, Position, Renderable, Wall};

/*
创建墙实体
*/
pub fn create_wall(world: &mut World, position: Position) {
  world.create_entity()
   .with(Position { z: 10, ..position })
   .with(Renderable::new_static("/images/wall.png".to_string()))
   .with(Immovable {})
   .with(Wall {})
   .build();
}

/*
创建地面实体
*/
pub fn create_floor(world: &mut World, position: Position) {
  world.create_entity()
      .with(Position { z: 5, ..position })
      .with(Renderable::new_static("/images/floor.png".to_string()))
      .build();
}

/*
创建玩家实体
*/
pub fn create_player(world: &mut World, position: Position) {
  world.create_entity()
   .with(Position { z: 10, ..position })
   .with(Renderable:: new(vec![
      "/images/player_1.png".to_string(), 
      "/images/player_2.png".to_string(), 
      "/images/player_3.png".to_string()]))
   .with(Movable {})
   .with(Player {})
   .build();
}

/*
创建箱子实体
*/
pub fn create_box(world: &mut World, position: Position, box_color: BoxColor) {
  world.create_entity()
   .with(Position { z: 10, ..position })
   .with(Renderable::new(vec![
      format!("/images/box_{}_1.png", box_color),
      format!("/images/box_{}_2.png", box_color)
    ]))
   .with(Movable {})
   .with(Box { color: box_color })
   .build();
}


/*
创建箱子位置实体
*/
pub fn create_box_spot(world: &mut World, position: Position, box_color: BoxColor) {
  world.create_entity()
   .with(Position { z: 9, ..position })
   .with(Renderable::new_static(format!("/images/box_spot_{}.png", box_color)))
   .with(BoxSpot { color: box_color })
   .build();
} 