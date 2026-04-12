use std::f32::consts::{FRAC_PI_2, TAU};

use macroquad::prelude::*;

pub enum EnemyType {
    Hexagon,
    Triangle,
    Square,
}

pub struct Enemy {
    position: Vec2,
    enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(position: Vec2, enemy_type: EnemyType) -> Self {
        Self {
            position,
            enemy_type,
        }
    }

    pub fn draw(&self) {
        match self.enemy_type {
            EnemyType::Hexagon => {
                draw_hexagon(self.position.x, self.position.y, 8.0, 1.0, true, RED, RED);
            }
            EnemyType::Triangle => {
                let (v1, v2, v3) = Self::equilateral_triangle_points(self.position, 16.0);
                draw_triangle(v1, v2, v3, RED);
            }
            EnemyType::Square => {
                draw_rectangle(self.position.x, self.position.y, 16.0, 16.0, RED);
            }
        }
    }

    fn equilateral_triangle_points(center: Vec2, radius: f32) -> (Vec2, Vec2, Vec2) {
        let v = |i: f32| center + Vec2::from_angle(-FRAC_PI_2 + i * TAU / 3.0) * radius;
        (v(0.0), v(1.0), v(2.0))
    }
}
