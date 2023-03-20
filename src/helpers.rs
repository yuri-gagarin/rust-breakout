use macroquad::prelude::{Rect, Vec2};
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
/* 
pub fn resolve_collision(first: &mut Rect, vector: &mut Vec2, second: &Rect) -> bool {
    /* 
    if let Some(_intersection) = first.intersect(*second) {
        vector.y = vector.y *  -1.0f32;
        return true;
    }
    false
    */
    let intersection = match first.intersect(*second) {
        Some(intersection) => intersection,
        None => return false,
    };
    let first_center = first.point() + first.size() * 0.5f32;
    let second_center = second.point() + second.size() * 0.5f32;
    let to = second_center - first_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // y bounce
            first.y -= to_signum.y * intersection.h;
            match to_signum.y > 0.0f32 {
                true => vector.y = -vector.y.abs(),
                false => vector.y = vector.y.abs(),
            }
        }
        false => {
            // x bounce //
            first.x -= to_signum.x * intersection.w;
            match  to_signum.x < 0f32 {
                true => vector.x = vector.x.abs(),
                false => vector.x = -vector.x.abs(),
            }
        }
    }
    false
}
*/
pub fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
  // early exit
  let intersection = match a.intersect(*b) {
      Some(intersection) => intersection,
      None => return false,
  };
  let a_center = a.point() + a.size() * 0.5f32;
  let b_center = b.point() + b.size() * 0.5f32;
  let to = b_center - a_center;
  let to_signum = to.signum();
  match intersection.w > intersection.h {
      true => {
          // bounce on y
          a.y -= to_signum.y * intersection.h;
          vel.y = -to_signum.y * vel.y.abs();
      }
      false => {
          // bounce on x
          a.x -= to_signum.x * intersection.w;
          vel.x = -to_signum.x * vel.x.abs();
      }
  }
  true
}