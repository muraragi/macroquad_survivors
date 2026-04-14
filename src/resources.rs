use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct FrameTime(pub f32);

#[derive(Resource)]
pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
}
