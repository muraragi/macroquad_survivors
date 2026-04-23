use bevy_ecs::prelude::*;
use macroquad::prelude::*;

mod consts;
mod enemy;
mod graphics;
mod movement;
mod player;
mod resources;
mod stats;
mod utils;
mod weapon;

use enemy::*;
use movement::*;
use player::*;
use resources::*;
use stats::*;
use weapon::*;

use crate::graphics::{draw_particles, update_particles};

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
    set_cursor_grab(true);

    let mut world = World::new();
    let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

    let player = world
        .spawn((
            Player,
            Position(screen_center),
            Speed(consts::speed::FAST),
            Health(consts::health::STRONG),
        ))
        .id();

    world.spawn((
        Weapon {
            projectile_velocity: 500.0,
            holder: player,
        },
        Damage(consts::damage::STARTING_PLAYER_DAMAGE),
    ));

    world.insert_resource(FrameTime(0.0));
    world.insert_resource(TimeElapsed(0.0));
    world.insert_resource(ScreenSize {
        width: screen_width(),
        height: screen_height(),
    });

    world.insert_resource(EnemySpawnTimer(Timer::new(1.0)));
    world.insert_resource(EnemyAttackTimer(Timer::new(0.5)));

    world.insert_resource(PlayerTarget(None));

    world.insert_resource(WeaponAttackTimer(Timer::new(0.2)));

    let mut schedule = Schedule::default();
    schedule.add_systems((
        player_controls,
        enemy_spawner,
        move_enemies.after(player_controls),
        draw_enemies.after(move_enemies).after(enemy_spawner),
        draw_player.after(move_enemies),
        enemy_player_collision
            .after(move_enemies)
            .after(player_controls),
        draw_player_health.after(enemy_player_collision),
        select_target.after(move_enemies),
        draw_reticle.after(select_target),
        fire_weapon.after(select_target),
        move_projectiles.after(fire_weapon).after(move_enemies),
        draw_projectiles.after(move_projectiles),
        projectile_enemy_collision
            .after(move_enemies)
            .after(move_projectiles),
        draw_target_health.after(projectile_enemy_collision),
        update_particles.after(projectile_enemy_collision),
        draw_particles.after(update_particles),
    ));

    loop {
        world.resource_mut::<FrameTime>().0 = get_frame_time();
        world.resource_mut::<TimeElapsed>().0 = get_time();
        let mut screen = world.resource_mut::<ScreenSize>();
        screen.width = screen_width();
        screen.height = screen_height();

        clear_background(Color {
            r: 0.0,
            g: 0.0,
            b: 0.08,
            a: 1.0,
        });

        schedule.run(&mut world);

        next_frame().await
    }
}
