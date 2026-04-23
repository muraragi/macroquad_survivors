use std::{
    f32::consts::{FRAC_PI_2, TAU},
    f64::consts::TAU as TAU_64,
};

use bevy_ecs::prelude::*;
use macroquad::{prelude::*, rand::gen_range};

use crate::{movement::Position, resources::FrameTime};

#[derive(Component)]
pub struct Particle {
    velocity: Vec2,
    lifetime: f32,
    size: f32,
    speed: f32,
}

pub fn create_explosion_particles() -> Vec<Particle> {
    let mut result: Vec<Particle> = vec![];

    for _ in 1..10 {
        let angle = gen_range(0.0, TAU);
        let speed = gen_range(50.0, 150.0);
        let size = gen_range(2.0, 4.0);
        let lifetime = gen_range(1.0, 2.2);
        let velocity = Vec2::from_angle(angle as f32);

        result.push(Particle {
            velocity,
            size,
            lifetime,
            speed,
        });
    }

    result
}

pub fn update_particles(
    particles: Query<(&mut Particle, &mut Position, Entity)>,
    frametime: Res<FrameTime>,
    mut commands: Commands,
) {
    for (mut particle, mut position, entity) in particles {
        particle.lifetime -= frametime.0;

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        position.0 += particle.velocity.normalize_or_zero() * particle.speed * frametime.0;
    }
}

pub fn draw_particles(particles: Query<(&mut Particle, &mut Position)>) {
    for (particle, position) in particles {
        let alpha = particle.lifetime.clamp(0.0, 1.0);
        let color = Color::new(1.0, 0.6, 0.2, alpha);
        let sides = get_equilateral_triangle_sides(position.0, particle.size);
        draw_triangle(sides.0, sides.1, sides.2, color);
    }
}

pub fn draw_reticle_at_pos(position: &Position) {
    let time_elapsed = get_time();
    let near_offset = (32.0 + 6.0 * (TAU_64 * 0.8 * time_elapsed).sin()) as f32;
    let far_offset = (52.0 + 6.0 * (TAU_64 * 0.8 * time_elapsed).sin()) as f32;

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

pub fn get_equilateral_triangle_sides(position: Vec2, size: f32) -> (Vec2, Vec2, Vec2) {
    let v = |i: f32| position + Vec2::from_angle(-FRAC_PI_2 + i * TAU / 3.0) * size;
    (v(0.0), v(1.0), v(2.0))
}
