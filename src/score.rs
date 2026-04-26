use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::resources::ScreenSize;

#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct Value(pub i32);

pub fn draw_score(score: Res<Score>, screen: Res<ScreenSize>) {
    let score_text = format!("Score: {}", score.0);
    draw_text(&score_text, (screen.width / 2.0) - 64.0, 36.0, 32.0, WHITE);
}
