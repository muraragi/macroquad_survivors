use bevy_ecs::prelude::*;
use macroquad::{prelude::*, rand::ChooseRandom};

use crate::resources::ScreenSize;

const LEVEL_SCORE_THRESHOLD: i32 = 8;

#[derive(Clone, Copy)]
pub enum Upgrade {
    Attack(f32),
    Speed(f32),
    FireRate(f32),
    Health(f32),
    Target,
}

impl Upgrade {
    pub fn label(&self) -> String {
        match self {
            Upgrade::Attack(attack_amount) => format!("+{} ATTACK", attack_amount),
            Upgrade::Speed(speed_amount) => format!("+{} SPEED", speed_amount),
            Upgrade::FireRate(fire_rate_amount) => format!("-{} FIRE TIMER", fire_rate_amount),
            Upgrade::Health(health_amount) => format!("+{} HEALTH", health_amount),
            Upgrade::Target => "+1 TARGET".into(),
        }
    }
}

#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Resource)]
pub struct UpgradesToChoose(pub [Upgrade; 3]);

#[derive(Component)]
pub struct Value(pub i32);

pub fn draw_score(score: Res<Score>, screen: Res<ScreenSize>) {
    let score_text = format!("Score: {}", score.0);
    let lvl_text = format!("Level: {}", level_for_score(score.0));
    draw_text(&score_text, (screen.width) - 136.0, 36.0, 32.0, WHITE);
    draw_text(&lvl_text, (screen.width) - 136.0, 72.0, 32.0, WHITE);
}

pub fn level_for_score(score: i32) -> i32 {
    ((score + LEVEL_SCORE_THRESHOLD) as f32 / LEVEL_SCORE_THRESHOLD as f32)
        .log2()
        .floor() as i32
}

pub fn get_random_upgrades() -> [Upgrade; 3] {
    let mut possible_upgrades: [Upgrade; 5] = [
        Upgrade::Attack(2.0),
        Upgrade::Speed(20.0),
        Upgrade::Health(10.0),
        Upgrade::FireRate(0.05),
        Upgrade::Target,
    ];

    possible_upgrades.shuffle();
    possible_upgrades[0..3].try_into().unwrap()
}
