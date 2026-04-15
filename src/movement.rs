use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Speed(pub f32);
