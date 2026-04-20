use macroquad::prelude::*;

pub fn seek_target(current_pos: Vec2, target_pos: Vec2, velocity: f32) -> Vec2 {
    let dir = target_pos - current_pos;
    dir.normalize_or_zero() * velocity
}

pub fn check_simple_collision(pos1: Vec2, pos2: Vec2, collision_range: f32) -> bool {
    let distance = (pos1 - pos2).length_squared();

    distance < collision_range * collision_range
}
