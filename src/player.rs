use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::movement::{Position, Speed};
use crate::resources::{FrameTime, ScreenSize};

pub const PLAYER_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Player;

pub fn player_controls(
    mut query: Query<(&mut Position, &Speed), With<Player>>,
    frame_time: Res<FrameTime>,
    screen: Res<ScreenSize>,
) {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    for (mut pos, speed) in &mut query {
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

        pos.0 += dir.normalize_or_zero() * speed.0 * frame_time.0;
        pos.0.x = clamp(pos.0.x, PLAYER_SIZE, screen.width - PLAYER_SIZE);
        pos.0.y = clamp(pos.0.y, PLAYER_SIZE, screen.height - PLAYER_SIZE);
    }
}

pub fn draw_player(query: Query<&Position, With<Player>>) {
    for pos in &query {
        draw_circle(
            pos.0.x,
            pos.0.y,
            PLAYER_SIZE,
            Color {
                r: 0.3,
                g: 0.0,
                b: 0.8,
                a: 1.0,
            },
        );
    }
}
