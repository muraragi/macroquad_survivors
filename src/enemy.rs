use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::consts::{damage, health, speed};
use crate::graphics::get_equilateral_triangle_sides;
use crate::movement::{Position, Speed};
use crate::player::{PLAYER_SIZE, Player};
use crate::resources::{FrameTime, ScreenSize, Timer};
use crate::stats::{Damage, Health};
use crate::utils::{check_simple_collision, seek_target};

const DAMAGE_RANGE: f32 = 8.0;

pub enum EnemyType {
    Hexagon,
    Triangle,
    Square,
}

impl EnemyType {
    fn random() -> Self {
        match gen_range(0_u32, 3) {
            0 => EnemyType::Hexagon,
            1 => EnemyType::Square,
            _ => EnemyType::Triangle,
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct EnemyAttackTimer(pub Timer);

#[derive(Component)]
pub struct Enemy {
    enemy_type: EnemyType,
}

struct EnemyStats {
    health: f32,
    damage: f32,
    speed: f32,
}

pub fn enemy_spawner(
    screen: Res<ScreenSize>,
    frame_time: Res<FrameTime>,
    mut command: Commands,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_timer.0.tick(frame_time.0) {
        let enemy_pos = Vec2 {
            x: gen_range(0.0_f32, screen.width),
            y: gen_range(0.0_f32, screen.height),
        };
        let enemy_type = EnemyType::random();
        let enemy_stats: EnemyStats = match enemy_type {
            EnemyType::Hexagon => EnemyStats {
                speed: speed::SLOW,
                health: health::MEDIUM,
                damage: damage::HEXAGON_DAMAGE,
            },
            EnemyType::Triangle => EnemyStats {
                speed: speed::MEDIUM,
                health: health::WEAK,
                damage: damage::TRIANGLE_DAMAGE,
            },
            EnemyType::Square => EnemyStats {
                speed: speed::MEDIUM,
                health: health::WEAK,
                damage: damage::SQUARE_DAMAGE,
            },
        };

        command.spawn((
            Enemy { enemy_type },
            Position(enemy_pos),
            Speed(enemy_stats.speed),
            Damage(enemy_stats.damage),
            Health(enemy_stats.health),
        ));
    }
}

pub fn move_enemies(
    mut enemies: Query<(&mut Position, &Speed), (With<Enemy>, Without<Player>)>,
    player: Query<&Position, With<Player>>,
    frame_time: Res<FrameTime>,
) {
    if let Ok(player_pos) = player.single() {
        let player_pos = player_pos.0;
        for (mut pos, speed) in &mut enemies {
            let movement = seek_target(pos.0, player_pos, speed.0) * frame_time.0;
            pos.0 += movement;
        }
    }
}

pub fn enemy_player_collision(
    enemies: Query<(&Position, &Damage), With<Enemy>>,
    mut player: Query<(&Position, &mut Health), With<Player>>,
    frame_time: Res<FrameTime>,
    mut enemy_attack_timer: ResMut<EnemyAttackTimer>,
) {
    if let Ok((player_pos, mut player_health)) = player.single_mut()
        && enemy_attack_timer.0.tick(frame_time.0)
    {
        for (enemy_pos, enemy_damage) in enemies {
            if check_simple_collision(player_pos.0, enemy_pos.0, PLAYER_SIZE + DAMAGE_RANGE) {
                player_health.0 -= enemy_damage.0;
            }
        }
    };
}

pub fn draw_enemies(query: Query<(&Position, &Enemy)>) {
    for (pos, enemy) in &query {
        match enemy.enemy_type {
            EnemyType::Hexagon => {
                draw_hexagon(pos.0.x, pos.0.y, 12.0, 1.0, true, RED, RED);
            }
            EnemyType::Triangle => {
                let sides = get_equilateral_triangle_sides(pos.0, PLAYER_SIZE);
                draw_triangle(sides.0, sides.1, sides.2, RED);
            }
            EnemyType::Square => {
                draw_rectangle(pos.0.x - 12.0, pos.0.y - 12.0, 24.0, 24.0, RED);
            }
        }
    }
}
