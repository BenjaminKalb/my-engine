use bevy_ecs::prelude::*;
use glam::Vec3;

#[derive(Component, Copy, Clone, Default)]
pub struct Position {
    pub value: Vec3,
    pub last_value: Vec3,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let v = Vec3::new(x, y, z);
        Self {
            value: v,
            last_value: v,
        }
    }

    pub fn from_vec3(v: Vec3) -> Self {
        Self {
            value: v,
            last_value: v,
        }
    }

    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.value.x, self.value.y, self.value.z)
    }
}