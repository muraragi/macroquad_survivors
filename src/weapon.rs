use bevy_ecs::prelude::*;
use macroquad::{color::WHITE, shapes::draw_circle};

use crate::{
    enemy::Enemy,
    graphics::{Particle, create_explosion_particles},
    movement::Position,
    player::PlayerTarget,
    resources::{FrameTime, Timer},
    score::{Score, Value},
    stats::{Damage, Health},
    utils::{check_simple_collision, seek_target},
};

const PROJECTILE_SIZE: f32 = 2.0;

#[derive(Component)]
pub struct Weapon {
    pub projectile_velocity: f32,
    pub holder: Entity,
    pub attack_timer: Timer,
}

impl Weapon {
    pub fn new(holder: Entity, projectile_velocity: f32, fire_rate: f32) -> Self {
        Weapon {
            projectile_velocity,
            holder,
            attack_timer: Timer::new(fire_rate),
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    velocity: f32,
    target: Entity,
}

pub fn fire_weapon(
    target: Res<PlayerTarget>,
    holders: Query<&Position>,
    weapons: Query<(&mut Weapon, &Damage)>,
    frame_time: Res<FrameTime>,
    mut commands: Commands,
) {
    if let Some(target) = target.0 {
        for (mut weapon, weapon_damage) in weapons {
            if weapon.attack_timer.tick(frame_time.0)
                && let Ok(holder_position) = holders.get(weapon.holder)
            {
                commands.spawn((
                    Position(holder_position.0),
                    Projectile {
                        velocity: weapon.projectile_velocity,
                        target,
                    },
                    Damage(weapon_damage.0),
                ));
            };
        }
    }
}

pub fn move_projectiles(
    projectiles: Query<(&Projectile, &mut Position, Entity), Without<Enemy>>,
    enemy_pos_query: Query<&Position, With<Enemy>>,
    frame_time: Res<FrameTime>,
    mut commands: Commands,
) {
    for (projectile, mut pos, entity_id) in projectiles {
        if let Ok(target_pos) = enemy_pos_query.get(projectile.target) {
            let movement = seek_target(pos.0, target_pos.0, projectile.velocity) * frame_time.0;
            pos.0 += movement
        } else {
            commands.entity(entity_id).despawn();
        }
    }
}

pub fn projectile_enemy_collision(
    projectiles: Query<(&Projectile, &Position, &Damage, Entity), Without<Enemy>>,
    mut enemy_query: Query<(&Position, &mut Health, &Value, Entity), With<Enemy>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
) {
    for (projectile, porjectile_pos, projectile_damage, projectile_entity_id) in projectiles {
        if let Ok((target_pos, mut target_health, target_value, enemy_entity_id)) =
            enemy_query.get_mut(projectile.target)
            && check_simple_collision(target_pos.0, porjectile_pos.0, PROJECTILE_SIZE + 8.0)
        {
            target_health.0 -= projectile_damage.0;
            commands.entity(projectile_entity_id).despawn();
            if target_health.0 <= 0.0 {
                commands.entity(enemy_entity_id).despawn();
                let particles = create_explosion_particles();
                commands.spawn_batch(
                    particles
                        .into_iter()
                        .map(|particle| (Position(target_pos.0), particle))
                        .collect::<Vec<(Position, Particle)>>(),
                );
                score.0 += target_value.0;
            }
        }
    }
}

pub fn draw_projectiles(projectiles: Query<&Position, With<Projectile>>) {
    for projectile_pos in projectiles {
        draw_circle(
            projectile_pos.0.x,
            projectile_pos.0.y,
            PROJECTILE_SIZE,
            WHITE,
        );
    }
}
