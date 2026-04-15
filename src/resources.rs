use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct FrameTime(pub f32);

#[derive(Resource)]
pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct Timer {
    elapsed: f32,
    interval: f32,
}

impl Timer {
    pub fn new(interval: f32) -> Self {
        Self {
            interval,
            elapsed: 0.0,
        }
    }

    pub fn tick(&mut self, delta: f32) -> bool {
        self.elapsed += delta;

        if self.elapsed >= self.interval {
            self.elapsed -= self.interval;
            return true;
        }

        false
    }
}
