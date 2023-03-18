use macroquad::prelude::{screen_width};
use super::player::{Player};

pub fn check_player_paddle_collison(player: &mut Player) {
  if player.rect.x < 0.0f32 {
      player.rect.x = 0.0f32;
  }
  if player.rect.x > screen_width() - player.rect.w {
      player.rect.x = screen_width() - player.rect.w;
  }
}