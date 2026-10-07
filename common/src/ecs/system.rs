use crate::ecs::world::World;

pub trait System {
    fn run(&mut self, world: &mut World);
}