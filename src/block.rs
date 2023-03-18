
use macroquad::prelude::{Rect, Vec2};
use macroquad::prelude::{draw_rectangle};
use super::constants::{BLOCK_SIZE};

pub struct Block {
  rect: Rect,
}
impl Block {
  pub fn new(position: Vec2) -> Self {
      Self {
          rect: Rect::new(position.x, position.y, BLOCK_SIZE.x, BLOCK_SIZE.y)
      }
  }
  pub fn draw(&self) {
      draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, macroquad::prelude::BLUE);
  }
}
