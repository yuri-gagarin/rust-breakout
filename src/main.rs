use macroquad::prelude::{Vec2, Rect};
use macroquad::prelude::{clear_background, next_frame, screen_width, screen_height};
use macroquad::shapes::draw_rectangle;

struct Player {
    rect: Rect,
}
impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32, screen_height() - 100f32, PLAYER_SIZE.x, PLAYER_SIZE.y)
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, macroquad::prelude::BLUE);
    }
 }
const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);

#[macroquad::main("Breakout")]
async fn main() {
    let player: Player = Player::new();
    loop {
        clear_background(macroquad::prelude::WHITE);
        player.draw();
        next_frame().await
    }
}
