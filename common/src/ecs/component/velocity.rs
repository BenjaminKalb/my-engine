use bevy_ecs::prelude::*;
use glam::Vec3;

#[derive(Component, Clone, Copy, Default)]
struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}