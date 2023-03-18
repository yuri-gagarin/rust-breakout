use macroquad::prelude::{rand, Rect, Vec2};
use macroquad::prelude::{draw_rectangle, screen_width, vec2};
use macroquad::prelude::{DARKGRAY};
use super::constants::{BALL_SIZE, BALL_SPEED};

pub struct Ball {
    rect: Rect,
    vector: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vector: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vector.x * dt * BALL_SPEED;
        self.rect.y += self.vector.y * dt * BALL_SPEED;
        // collision detection //
        if self.rect.x < 0f32 {
            self.vector.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vector.x = -1f32;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}