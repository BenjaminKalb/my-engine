use bevy_ecs::prelude::*;
use glam::Vec3;

#[derive(Component, Clone, Copy, Default)]
pub struct BoundingBox {
    pub size: Vec3,
}

impl BoundingBox {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self {
            size: Vec3::new(width, height, depth),
        }
    }

    pub fn dimensions(&self) -> (f32, f32, f32) {
        (self.size.x, self.size.y, self.size.z)
    }

    pub fn half_size(&self) -> Vec3 {
        self.size * 0.5
    }
}