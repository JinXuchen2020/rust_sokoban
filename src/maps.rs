use specs::World;

use crate::{components::{BoxColor, Position}, entities::{create_box, create_box_spot, create_floor, create_player, create_wall}};

pub fn initialize_level(world: &mut World, map_string: &str) {
  load_map(world, map_string.to_string());
}

fn load_map(world: &mut World, map: String) {
  let lines: Vec<&str> = map.trim().split('\n').map(|line| line.trim()).collect();

  for (y, line) in lines.iter().enumerate() {
    let columns: Vec<&str> = line.split(' ').collect();
    for (x, c) in columns.iter().enumerate() {
      let position = Position { x: x as u8, y: y as u8, z: 0 };
      match *c {
        "." => create_floor(world, position),
        "W" => {
          create_floor(world, position);
          create_wall(world, position);
        } 
        "P" => {
          create_floor(world, position);
          create_player(world, position);
        }
        "BB" =>{
          create_floor(world, position);
          create_box(world, position, BoxColor::Blue);
        }
        "RB" =>{
          create_floor(world, position);
          create_box(world, position, BoxColor::Red);
        }
        "BS" => {
          create_floor(world, position);
          create_box_spot(world, position, BoxColor::Blue);
        }
        "RS" => {
          create_floor(world, position);
          create_box_spot(world, position, BoxColor::Red);
        }
        "N" => (),
        _ => panic!("unrecognized map item {}", c),
      }
    }
  } 
}