use bevy_ecs::prelude::*;
use macroquad::prelude::*;

pub const PLAYER_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub enum EnemyShape {
    Hexagon,
    Triangle,
    Square,
}
