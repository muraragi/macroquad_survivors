use bevy_ecs::prelude::*;
use macroquad::{
    miniquad::window::screen_size,
    prelude::*,
    ui::{Skin, hash, root_ui},
};

mod consts;
mod enemy;
mod graphics;
mod movement;
mod player;
mod resources;
mod stats;
mod ui;
mod utils;
mod weapon;

use enemy::*;
use movement::*;
use player::*;
use resources::*;
use stats::*;
use weapon::*;

use crate::{
    graphics::{Particle, draw_particles, update_particles},
    ui::{get_skin, render_menu},
};

fn get_window_config() -> Conf {
    Conf {
        window_title: "Macroquad Survivors".to_string(),
        window_width: 1280,
        window_height: 1024,
        ..Default::default()
    }
}

#[derive(Event)]
pub struct GameStateChange(pub GameState);

#[derive(Resource, Copy, Clone)]
pub enum GameState {
    Menu,
    Running,
}

#[macroquad::main(get_window_config)]
async fn main() {
    // set_cursor_grab(true);
    let bg_color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.08,
        a: 1.0,
    };

    let skin = get_skin(&bg_color);

    root_ui().push_skin(&skin);

    let mut world = World::new();

    world.insert_resource(GameState::Menu);
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

    let mut game_schedule = Schedule::default();
    game_schedule.add_systems((
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

    let render_menu_id = world.register_system(render_menu);
    let mut menu_schedule = Schedule::default();
    menu_schedule.add_systems(render_menu);

    world.add_observer(
        |state: On<GameStateChange>, screen: Res<ScreenSize>, mut commands: Commands| match state.0
        {
            GameState::Menu => {
                commands.queue(|world: &mut World| {
                    let to_despawn: Vec<Entity> = world
                        .query_filtered::<Entity, Or<(
                            With<Player>,
                            With<Enemy>,
                            With<Projectile>,
                            With<Weapon>,
                            With<Particle>,
                        )>>()
                        .iter(world)
                        .collect();
                    for e in to_despawn {
                        world.despawn(e);
                    }
                });
            }
            GameState::Running => {
                let screen_center = Vec2::new(screen.width / 2.0, screen.height / 2.0);
                let player = commands
                    .spawn((
                        Player,
                        Position(screen_center),
                        Speed(consts::speed::FAST),
                        Health(consts::health::STRONG),
                    ))
                    .id();

                commands.spawn((
                    Weapon {
                        projectile_velocity: 500.0,
                        holder: player,
                    },
                    Damage(consts::damage::STARTING_PLAYER_DAMAGE),
                ));
            }
        },
    );

    loop {
        let state = *world.resource::<GameState>();

        world.resource_mut::<FrameTime>().0 = get_frame_time();
        world.resource_mut::<TimeElapsed>().0 = get_time();
        let mut screen = world.resource_mut::<ScreenSize>();
        screen.width = screen_width();
        screen.height = screen_height();
        clear_background(bg_color);

        match state {
            GameState::Menu => {
                world
                    .run_system(render_menu_id)
                    .expect("render menu failed");
            }
            GameState::Running => {
                game_schedule.run(&mut world);
            }
        }

        next_frame().await
    }
}
