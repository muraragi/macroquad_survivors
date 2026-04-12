use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 16.0;

pub struct Player {
    position: Vec2,
    speed: f32,
}

impl Player {
    pub fn new(position: Vec2, speed: f32) -> Self {
        Player { position, speed }
    }

    pub fn draw(&self) {
        draw_circle(
            self.position.x,
            self.position.y,
            PLAYER_SIZE,
            Color {
                r: 0.3,
                g: 0.0,
                b: 0.8,
                a: 1.0,
            },
        );
    }

    pub fn handle_controls(&mut self, frame_time: f32, screen_width: f32, screen_height: f32) {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        let mut dir = Vec2::ZERO;

        if is_key_down(KeyCode::Right) {
            dir.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            dir.y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            dir.y -= 1.0;
        }

        self.position += dir.normalize_or_zero() * self.speed * frame_time;
        self.position.x = clamp(self.position.x, PLAYER_SIZE, screen_width - PLAYER_SIZE);
        self.position.y = clamp(self.position.y, PLAYER_SIZE, screen_height - PLAYER_SIZE);
    }
}
