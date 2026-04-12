use macroquad::prelude::*;
mod enemy;

mod player;
use player::Player;

use crate::enemy::{Enemy, EnemyType};

mod speed {
    pub const SLOW: f32 = 100.0;
    pub const MEDIUM: f32 = 150.0;
    pub const FAST: f32 = 200.0;
}

fn get_window_config() -> Conf {
    Conf {
        window_title: "Macroquad Survivors".to_string(),
        window_width: 1280,
        window_height: 1024,
        ..Default::default()
    }
}

#[macroquad::main(get_window_config)]
async fn main() {
    let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut player = Player::new(screen_center, speed::FAST);

    let mut enemies: Vec<Enemy> = vec![
        Enemy::new(Vec2::new(0.0, 200.0) + screen_center, EnemyType::Triangle),
        Enemy::new(Vec2::new(200.0, -200.0) + screen_center, EnemyType::Square),
        Enemy::new(
            Vec2::new(-200.0, -200.0) + screen_center,
            EnemyType::Hexagon,
        ),
    ];

    set_cursor_grab(true);

    loop {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let frame_time = get_frame_time();

        clear_background(Color {
            r: 0.0,
            g: 0.0,
            b: 0.08,
            a: 1.0,
        });

        player.handle_controls(frame_time, screen_width, screen_height);
        player.draw();

        for enemy in &enemies {
            enemy.draw();
        }

        next_frame().await
    }
}
