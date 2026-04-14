use std::f32::consts::{FRAC_PI_2, TAU};

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::components::*;
use crate::resources::*;

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

pub fn enemy_movement(
    mut enemies: Query<(&mut Position, &Speed), (With<EnemyShape>, Without<Player>)>,
    player: Query<&Position, With<Player>>,
    frame_time: Res<FrameTime>,
) {
    let Ok(player_pos) = player.single() else {
        return;
    };
    let player_pos = player_pos.0;
    for (mut pos, speed) in &mut enemies {
        let dir = player_pos - pos.0;
        pos.0 += dir.normalize_or_zero() * speed.0 * frame_time.0;
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

pub fn draw_enemies(query: Query<(&Position, &EnemyShape)>) {
    for (pos, shape) in &query {
        match shape {
            EnemyShape::Hexagon => {
                draw_hexagon(pos.0.x, pos.0.y, 8.0, 1.0, true, RED, RED);
            }
            EnemyShape::Triangle => {
                let v =
                    |i: f32| pos.0 + Vec2::from_angle(-FRAC_PI_2 + i * TAU / 3.0) * PLAYER_SIZE;
                draw_triangle(v(0.0), v(1.0), v(2.0), RED);
            }
            EnemyShape::Square => {
                draw_rectangle(pos.0.x, pos.0.y, 16.0, 16.0, RED);
            }
        }
    }
}
