use bevy_ecs::prelude::*;
use macroquad::prelude::*;

mod consts;
mod enemy;
mod graphics;
mod movement;
mod player;
mod resources;
mod stats;
mod weapon;

use enemy::*;
use movement::*;
use player::*;
use resources::*;
use stats::*;
use weapon::*;

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

    world.spawn((
        Player,
        Weapon {
            damage: 1.0,
            projectile_velocity: 500.0,
        },
        Position(screen_center),
        Speed(consts::speed::FAST),
        Health(consts::health::STRONG),
        Damage(consts::damage::STARTING_PLAYER_DAMAGE),
    ));

    world.insert_resource(FrameTime(0.0));
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
        enemy_movement.after(player_controls),
        draw_enemies.after(enemy_movement).after(enemy_spawner),
        draw_player.after(enemy_movement),
        enemy_player_collision
            .after(enemy_movement)
            .after(player_controls),
        player_health_ui.after(enemy_player_collision),
        seek_target.after(enemy_movement),
        draw_reticle.after(seek_target),
        fire_weapon.after(seek_target),
        move_projectiles.after(fire_weapon).after(enemy_movement),
        draw_projectiles.after(move_projectiles),
    ));

    loop {
        world.resource_mut::<FrameTime>().0 = get_frame_time();
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
