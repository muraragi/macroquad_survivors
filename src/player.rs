use std::cmp::Ordering;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::GameState;
use crate::enemy::Enemy;
use crate::graphics::draw_reticle_at_pos;
use crate::movement::{Position, Speed};
use crate::observers::GameStateChange;
use crate::resources::{FrameTime, ScreenSize};
use crate::stats::Health;
use crate::weapon::Weapon;

pub const PLAYER_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerTargets(pub Vec<Entity>);

pub fn player_controls(
    mut query: Query<(&mut Position, &Speed), With<Player>>,
    frame_time: Res<FrameTime>,
    screen: Res<ScreenSize>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
) {
    if is_key_pressed(KeyCode::Escape) {
        *game_state = GameState::Menu;
        commands.trigger(GameStateChange(*game_state));
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

pub fn draw_player_health(
    player: Query<&Health, With<Player>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
) {
    if let Ok(health) = player.single() {
        if health.0 <= 0.0 {
            *game_state = GameState::Menu;
            commands.trigger(GameStateChange(*game_state));
        }

        let health_text = format!("Health: {}", health.0);
        draw_text(&health_text, 24.0, 36.0, 32.0, GREEN);
    }
}

pub fn select_targets(
    enemies: Query<(Entity, &Position), With<Enemy>>,
    player_pos: Query<&Position, With<Player>>,
    weapon: Query<&Weapon>,
    mut targets: ResMut<PlayerTargets>,
) {
    if let Ok(player_pos) = player_pos.single()
        && let Ok(weapon) = weapon.single()
    {
        let targets_required = weapon.max_targets as usize - targets.0.len();

        if targets_required > 0 {
            let mut candidates = enemies
                .iter()
                .filter(|(enemy_entity, _)| !targets.0.iter().any(|target| target == enemy_entity))
                .collect::<Vec<(Entity, &Position)>>();

            candidates.sort_by(|(_, pos_a), (_, pos_b)| {
                let dist_a = (pos_a.0 - player_pos.0).length_squared();
                let dist_b = (pos_b.0 - player_pos.0).length_squared();

                dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Greater)
            });

            targets.0.extend(
                candidates
                    .iter()
                    .map(|(entity, _)| *entity)
                    .take(targets_required),
            );
        }
    }
}

pub fn draw_reticle(targets: Res<PlayerTargets>, transform_query: Query<&Position, With<Enemy>>) {
    for target in &targets.0 {
        if let Ok(target_position) = transform_query.get(*target) {
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
