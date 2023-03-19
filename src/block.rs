
use macroquad::prelude::{Rect, Vec2};
use macroquad::prelude::{draw_rectangle};
use super::constants::{BLOCK_SIZE};

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
}
impl Block {
  pub fn new(position: Vec2) -> Self {
      Self {
          rect: Rect::new(position.x, position.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
          lives: 2,
      }
  }
  pub fn draw(&self) {
      let color = match self.lives {
        2 => macroquad::prelude::RED,
        _ => macroquad::prelude::ORANGE,
      };
      draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
  }
}
