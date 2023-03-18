use macroquad::prelude::{KeyCode, Rect};
use macroquad::prelude::{draw_rectangle, is_key_down, screen_width, screen_height};
//
use super::constants::{PLAYER_SIZE, PLAYER_SPEED};
use super::helpers::{check_player_paddle_collison};

pub struct Player {
    pub rect: Rect,
}
impl Player {
  pub fn new() -> Self {
      Self {
          rect: Rect::new(screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32, screen_height() - 100f32, PLAYER_SIZE.x, PLAYER_SIZE.y)
      }
  }
  pub fn update(&mut self, dt: f32) {
      let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
          (true, false) => -1.0f32,
          (false, true) => 1.0f32,
          _             => 0.0f32,
      };
      self.rect.x += x_move * dt * PLAYER_SPEED;
      // player paddle collision handler //
      check_player_paddle_collison(self);
  }
  pub fn draw(&self) {
      draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, macroquad::prelude::BLUE);
  }
}