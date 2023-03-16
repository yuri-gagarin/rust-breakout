use macroquad::prelude::{Vec2, KeyCode, Rect};
use macroquad::prelude::{clear_background, get_frame_time, next_frame, screen_width, screen_height, is_key_down, vec2};
use macroquad::shapes::draw_rectangle;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);
const PLAYER_SPEED: f32 = 400.0;

fn check_player_paddle_collison(player: &mut Player) {
    if player.rect.x < 0.0f32 {
        player.rect.x = 0.0f32;
    }
    if player.rect.x > screen_width() - player.rect.w {
        player.rect.x = screen_width() - player.rect.w;
    }
}
struct Player {
    rect: Rect,
}
struct Block {
    rect: Rect,
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

#[macroquad::main("Breakout")]
async fn main() {
    let mut player: Player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();

    let (width, height) = (6, 6);
    let padding: f32 = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_position = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 
        50.0f32
    );
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_position + vec2(block_x, block_y)));
    }
    loop {
        player.update(get_frame_time());
        clear_background(macroquad::prelude::WHITE);
        player.draw();
        // iterate all blocks and draw //
        for block in blocks.iter() {
            block.draw();
        }
        next_frame().await
    }
}
