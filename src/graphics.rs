use std::f64::consts::TAU;

use macroquad::prelude::*;

use crate::movement::Position;

pub fn draw_reticle_at_pos(position: &Position) {
    let time_elapsed = get_time();
    let near_offset = (32.0 + 6.0 * (TAU * 0.8 * time_elapsed).sin()) as f32;
    let far_offset = (52.0 + 6.0 * (TAU * 0.8 * time_elapsed).sin()) as f32;

    let reticle_points: Vec<(Vec2, Vec2)> = vec![
        (
            Vec2::new(position.0.x - far_offset, position.0.y),
            Vec2::new(position.0.x - near_offset, position.0.y),
        ),
        (
            Vec2::new(position.0.x + near_offset, position.0.y),
            Vec2::new(position.0.x + far_offset, position.0.y),
        ),
        (
            Vec2::new(position.0.x, position.0.y - far_offset),
            Vec2::new(position.0.x, position.0.y - near_offset),
        ),
        (
            Vec2::new(position.0.x, position.0.y + near_offset),
            Vec2::new(position.0.x, position.0.y + far_offset),
        ),
    ];

    for point in reticle_points {
        draw_line(point.0.x, point.0.y, point.1.x, point.1.y, 3.0, GREEN);
    }
}
