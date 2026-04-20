use std::cmp::Ordering;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::enemy::Enemy;
use crate::graphics::draw_reticle_at_pos;
use crate::movement::{Position, Speed};
use crate::resources::{FrameTime, ScreenSize};
use crate::stats::Health;

pub const PLAYER_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerTarget(pub Option<Entity>);

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

pub fn player_health_ui(player: Query<(Entity, &Health), With<Player>>, mut commands: Commands) {
    if let Ok((player_entity, health)) = player.single() {
        if health.0 <= 0.0 {
            commands.entity(player_entity).despawn();
        }

        let health_text = format!("Health: {}", health.0);
        draw_text(&health_text, 24.0, 36.0, 32.0, WHITE);
    }
}

pub fn select_target(
    enemies: Query<(Entity, &Position), With<Enemy>>,
    player_pos: Query<&Position, With<Player>>,
    mut target: ResMut<PlayerTarget>,
) {
    if let Ok(player_pos) = player_pos.single() {
        if let Some(closest_enemy) = enemies.iter().min_by(|(_, a_pos), (_, b_pos)| {
            let dist_a = (a_pos.0 - player_pos.0).length_squared();
            let dist_b = (b_pos.0 - player_pos.0).length_squared();

            dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Greater)
        }) {
            target.0 = Some(closest_enemy.0);
        } else {
            target.0 = None;
        }
    }
}

pub fn draw_reticle(target: Res<PlayerTarget>, transform_query: Query<&Position, With<Enemy>>) {
    if let Some(target_entity) = target.0 {
        if let Ok(target_position) = transform_query.get(target_entity) {
            draw_reticle_at_pos(target_position);
        }
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
