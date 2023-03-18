mod constants;
mod player;
mod block;
mod ball;
mod helpers;
//
use macroquad::prelude::{clear_background, get_frame_time, next_frame, screen_height, screen_width, vec2};
use constants::{BLOCK_SIZE};
use player::Player;
use block::Block;
use ball::Ball;
//
#[macroquad::main("Breakout")]
async fn main() {
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
        player.update(get_frame_time());
        // ball //
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }
        clear_background(macroquad::prelude::WHITE);
        // draw player //
        player.draw();
        // iterate all blocks and draw //
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }
        next_frame().await
    }
}
