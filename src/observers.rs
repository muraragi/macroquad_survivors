use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::{
    GameState, consts,
    enemy::Enemy,
    graphics::Particle,
    movement::{Position, Speed},
    player::Player,
    resources::ScreenSize,
    score::Score,
    stats::{Damage, Health},
    weapon::{Projectile, Weapon},
};

#[derive(Event)]
pub struct GameStateChange(pub GameState);

#[derive(Event)]
pub struct LevelUp;

pub fn setup_observers(world: &mut World) {
    world.add_observer(
        |state: On<GameStateChange>, screen: Res<ScreenSize>, mut commands: Commands| match state.0
        {
            GameState::Menu => {
                commands.queue(|world: &mut World| {
                    world.resource_mut::<Score>().0 = 0;

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
                    Weapon::new(player, 500.0, 0.2),
                    Damage(consts::damage::STARTING_PLAYER_DAMAGE),
                ));
            }
        },
    );

    world.add_observer(|_: On<LevelUp>| {
        println!("Level Up!");
    });
}
