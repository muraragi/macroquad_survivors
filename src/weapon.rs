use bevy_ecs::prelude::*;
use macroquad::{color::WHITE, shapes::draw_circle};

use crate::{
    enemy::Enemy,
    movement::Position,
    player::{Player, PlayerTarget},
    resources::{FrameTime, Timer},
};

#[derive(Resource)]
pub struct WeaponAttackTimer(pub Timer);

// TODO: WeaponAttackTimer based fire_rate
#[derive(Component)]
pub struct Weapon {
    pub damage: f32,
    // fire_rate: f32,
    pub projectile_velocity: f32,
}

#[derive(Component)]
pub struct Projectile {
    damage: f32,
    velocity: f32,
    target: Entity,
}

pub fn fire_weapon(
    target: Res<PlayerTarget>,
    player: Query<(&Position, &Weapon), With<Player>>,
    mut attack_timer: ResMut<WeaponAttackTimer>,
    frame_time: Res<FrameTime>,
    mut commands: Commands,
) {
    if let Ok((player_position, player_weapon)) = player.single()
        && attack_timer.0.tick(frame_time.0)
        && let Some(target) = target.0
    {
        commands.spawn((
            Position(player_position.0),
            Projectile {
                damage: player_weapon.damage,
                velocity: player_weapon.projectile_velocity,
                target,
            },
        ));
    }
}

pub fn move_projectiles(
    projectiles: Query<(&Projectile, &mut Position, Entity), Without<Enemy>>,
    enemy_pos_query: Query<&Position, With<Enemy>>,
    frame_time: Res<FrameTime>,
    mut commands: Commands,
) {
    for (projectile, mut pos, entity_id) in projectiles {
        // let player_pos = player_pos.0;
        // for (mut pos, speed) in &mut enemies {
        //     let dir = player_pos - pos.0;
        //     pos.0 += dir.normalize_or_zero() * speed.0 * frame_time.0;
        // }
        if let Ok(target_pos) = enemy_pos_query.get(projectile.target) {
            let dir = target_pos.0 - pos.0;
            pos.0 += dir.normalize_or_zero() * projectile.velocity * frame_time.0;
        } else {
            commands.entity(entity_id).despawn();
        }
    }
}

pub fn draw_projectiles(projectiles: Query<&Position, With<Projectile>>) {
    for projectile_pos in projectiles {
        draw_circle(projectile_pos.0.x, projectile_pos.0.y, 2.0, WHITE);
    }
}
