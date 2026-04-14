use bevy_ecs::prelude::*;
use macroquad::prelude::*;

mod components;
mod consts;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

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

    world.spawn((Player, Position(screen_center), Speed(consts::speed::FAST)));

    let enemies = [
        (
            EnemyShape::Triangle,
            Vec2::new(0.0, 200.0),
            consts::speed::FAST,
        ),
        (
            EnemyShape::Square,
            Vec2::new(200.0, -200.0),
            consts::speed::MEDIUM,
        ),
        (
            EnemyShape::Hexagon,
            Vec2::new(-200.0, -200.0),
            consts::speed::SLOW,
        ),
    ];

    for (shape, offset, speed) in enemies {
        world.spawn((shape, Position(offset + screen_center), Speed(speed)));
    }

    world.insert_resource(FrameTime(0.0));
    world.insert_resource(ScreenSize {
        width: screen_width(),
        height: screen_height(),
    });

    let mut schedule = Schedule::default();
    schedule.add_systems((
        player_controls,
        enemy_movement.after(player_controls),
        draw_player.after(enemy_movement),
        draw_enemies.after(enemy_movement),
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
