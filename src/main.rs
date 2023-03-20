mod constants;
mod player;
mod block;
mod ball;
mod helpers;
use helpers::resolve_collision;
//
use macroquad::prelude::{clear_background, draw_text_ex, get_frame_time, is_key_pressed, load_ttf_font, next_frame, screen_height, screen_width, vec2};
use macroquad::prelude::{KeyCode,  TextParams };
use constants::{BLOCK_SIZE};
use player::Player;
use block::Block;
use ball::Ball;
//
#[macroquad::main("Breakout")]
async fn main() {
    let font = load_ttf_font("res/Heebo-Variable.ttf").await.unwrap();
    let mut score: u16 = 0;
    let mut player: Player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();

    let (width, height) = (6, 6);
    let padding: f32 = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_position = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 
        50.0f32
    );
    // draw the blocks //
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_position + vec2(block_x, block_y)));
    }
    // draw the ball ///
    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
    loop {
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
        }
        player.update(get_frame_time());
        // ball //
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }
        for ball in balls.iter_mut() {
            resolve_collision(&mut ball.rect, &mut ball.vector, &player.rect);
            for block in blocks.iter_mut() {
                if resolve_collision(&mut ball.rect, &mut ball.vector, &block.rect) {
                    block.lives -= 1;
                    score += 1;
                    print!("Block lives: {}", block.lives);
                }
            }
        }
        // check if block shoud exists //
        blocks.retain(|block| block.lives > 0);
    
        clear_background(macroquad::prelude::WHITE);
        player.draw();
        // 
        // draw player //
        // iterate all blocks and draw //
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }
        draw_text_ex(&format!("SCORE: {}", score), screen_width() * 0.5f32, 40.0f32, TextParams { font, font_size: 30u16, color: macroquad::prelude::BLACK, ..Default::default() });
        next_frame().await
    }
}
