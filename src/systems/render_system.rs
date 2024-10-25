use std::{collections::HashMap, time::Duration};

use ggez::{glam::Vec2, graphics::{self, Canvas, Color, DrawParam, Drawable, Image, InstanceArray}, Context};
use specs::{Join, Read, ReadStorage, System};

use crate::{components::{Position, Renderable, RenderableKind}, constants::TILE_WIDTH, resources::{Gameplay, Time}};

pub struct RenderSystem<'a>{
  context: &'a mut Context
}

impl<'a> RenderSystem<'a> {
  pub fn new(context: &'a mut Context) -> Self {
    Self {
      context
    }
  }

  pub fn draw_text(&mut self, canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
    let text = graphics::Text::new(text_string);
    let destination = Vec2::new(x, y);
    let color = Color::new(0.0, 0.0, 0.0, 1.0);

    let draw_params = DrawParam::new().dest(destination).color(color);
    text.draw(canvas, draw_params);
  }  

  pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
    let path_index = match renderable.kind() {
        RenderableKind::Static => {
          // We only have one image, so we just return that
          0
        }
        RenderableKind::Animated => {
          // If we have multiple, we want to select the right one based on the delta time.
          // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
          // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
          // we technically are on the next iteration of the loop (or on 0), but we will let
          // the renderable handle this logic of wrapping frames.
          ((delta.as_millis() % 1000) / 250) as usize
        }
    };

    let image_path = renderable.path(path_index);
    Image::from_path(self.context, image_path).expect("expected image")
  }

  #[allow(dead_code)]
  pub fn get_image_path (&self, renderable: &Renderable, delta: Duration) -> String {
    let path_index = match renderable.kind() {
        RenderableKind::Static => {
          // We only have one image, so we just return that
          0
        }
        RenderableKind::Animated => {
          // If we have multiple, we want to select the right one based on the delta time.
          // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
          // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
          // we technically are on the next iteration of the loop (or on 0), but we will let
          // the renderable handle this logic of wrapping frames.
          ((delta.as_millis() % 1000) / 250) as usize
        }
    };

    renderable.path(path_index)
  }

  pub fn draw_images_batch(&mut self, canvas: &mut Canvas, image_path: &str, draw_params: &Vec<DrawParam>) {
    let image = Image::from_path(self.context, image_path).expect("expected image");
    let mut instance_array = InstanceArray::new_ordered(self.context, image);            
      
    for draw_param in draw_params.iter() {        
      instance_array.push(*draw_param);
    }

    instance_array.draw(canvas, DrawParam::new());
  }
}

impl<'a> System<'a> for RenderSystem<'a> {
  type SystemData = (
    Read<'a, Gameplay>, 
    Read<'a, Time>,
    ReadStorage<'a, Position>, 
    ReadStorage<'a, Renderable>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (gameplay, time, positions, renderables) = data;

    // Clearing the screen (this gives us the background colour)
    let mut canvas = graphics::Canvas::from_frame(self.context, Color::WHITE);

    let canvas_mut = &mut canvas;

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
    rendering_data.sort_by_key(|&k| k.0.z);

    let rendering_batches: HashMap<String, Vec<DrawParam>> = HashMap::new();

    // Iterate through all pairs of positions & renderables, load the image
    // and draw it at the specified position.
    for (position, renderable) in rendering_data.iter() {
        // Load the image
        let image = self.get_image(renderable, time.delta);
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        image.draw(canvas_mut, DrawParam::new().dest(Vec2::new(x, y)));
    }

    // Draw all the batches in the correct order    
    for (image_path, draw_params) in rendering_batches.iter() {
      self.draw_images_batch(canvas_mut, image_path, draw_params);
    }

    self.draw_text(canvas_mut, &gameplay.state.to_string(), 525.0, 80.0);
    self.draw_text(canvas_mut, &gameplay.moves_count.to_string(), 525.0, 100.0);
    let fps = format!("FPS: {:.0}", self.context.time.fps());
    self.draw_text(canvas_mut, &fps, 525.0, 120.0);
        

    // Finally, present the context, this will actually display everything
    // on the screen. 
    canvas.finish(self.context).expect("expected to finish");
  }
}