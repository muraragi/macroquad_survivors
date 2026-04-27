use bevy_ecs::prelude::*;
use macroquad::{prelude::*, ui::root_ui};

mod consts;
mod enemy;
mod graphics;
mod movement;
mod observers;
mod player;
mod resources;
mod score;
mod stats;
mod ui;
mod utils;
mod weapon;

use enemy::*;
use player::*;
use resources::*;
use weapon::*;

use crate::{
    graphics::{draw_particles, update_particles},
    observers::setup_observers,
    score::{Score, draw_score},
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
    world.insert_resource(Score(0));

    world.insert_resource(EnemySpawnTimer(Timer::new(1.0)));
    world.insert_resource(EnemyAttackTimer(Timer::new(0.5)));

    world.insert_resource(PlayerTarget(None));

    let mut game_schedule = Schedule::default();
    game_schedule.add_systems(
        (
            (player_controls, enemy_spawner),
            (move_enemies, select_target).chain(),
            fire_weapon,
            move_projectiles,
            (enemy_player_collision, projectile_enemy_collision),
            update_particles,
            (
                draw_enemies,
                draw_player,
                draw_projectiles,
                draw_reticle,
                draw_player_health,
                draw_target_health,
                draw_score,
                draw_particles,
            ),
        )
            .chain(),
    );

    let render_menu_id = world.register_system(render_menu);
    let mut menu_schedule = Schedule::default();
    menu_schedule.add_systems(render_menu);

    setup_observers(&mut world);

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
