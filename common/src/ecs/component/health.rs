use bevy_ecs::prelude::*;

#[derive(Component, Copy, Clone, Default)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value: value.max(0.0) }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.value = (self.value - amount)
            .max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.value = (self.value + amount)
            .max(0.0);
    }

    pub fn is_alive(&self) -> bool {
        self.value > 0.0
    }
}