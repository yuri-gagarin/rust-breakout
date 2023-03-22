mod constants;
mod player;
mod block;
mod ball;
mod helpers;
use helpers::resolve_collision;
//
use macroquad::prelude::{clear_background, get_frame_time, is_key_pressed, load_ttf_font, next_frame, screen_height, screen_width, vec2};
use macroquad::prelude::{KeyCode};
use constants::{BLOCK_SIZE};
use player::Player;
use block::Block;
use ball::Ball;
//

pub enum GameState {
    Menu,
    Game,
    Completed,
    Failed,
}

#[macroquad::main("Breakout")]
async fn main() {
    let font = load_ttf_font("res/Heebo-Variable.ttf").await.unwrap();
    // game state //
    let mut game_state = GameState::Menu;
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
                    if  block.lives == 0 {
                        score += 10; 
                    }
                    score += 1;
                }
            }
        }
        // ball should exist only if in screen //
        // calculate players lives //
        let balls_len = balls.len(); 
        balls.retain(|ball| ball.rect.y < screen_height());
        let removed_balls = balls_len - balls.len();
        if removed_balls > 0 {
            player.lives -= 1;
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

        match game_state {
            GameState::Menu => {
                helpers::draw_main_text("Press SPACE BAR to start", font, 30u16, macroquad::prelude::BLACK);
            },
            GameState::Game => {
                // draw score text and lives text //
                helpers::draw_score_text(score, font, 30u16, macroquad::prelude::BLACK);
                helpers::draw_lives_text(player.lives, font, 30u16, macroquad::prelude::BLACK);
            },
            GameState::Completed => {
                helpers::draw_main_text(&format!("You WON! Score: {}", score), font, 30u16, macroquad::prelude::BLACK);
            },
            GameState::Failed => {
                helpers::draw_main_text(&format!("GAME OVER! Score: {}", score), font, 30u16, macroquad::prelude::BLACK);
            }
        }
      
        //
        next_frame().await
    }
}
