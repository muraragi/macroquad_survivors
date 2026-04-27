use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::resources::ScreenSize;

const LEVEL_SCORE_THRESHOLD: i32 = 8;

#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct Value(pub i32);

pub fn draw_score(score: Res<Score>, screen: Res<ScreenSize>) {
    let score_text = format!("Score: {}", score.0);
    let lvl_text = format!("Level: {}", level_for_score(score.0));
    draw_text(&score_text, (screen.width / 2.0) - 64.0, 36.0, 32.0, WHITE);
    draw_text(&lvl_text, (screen.width / 2.0) - 64.0, 72.0, 32.0, WHITE);
}

pub fn level_for_score(score: i32) -> i32 {
    ((score + LEVEL_SCORE_THRESHOLD) as f32 / LEVEL_SCORE_THRESHOLD as f32)
        .log2()
        .floor() as i32
}
