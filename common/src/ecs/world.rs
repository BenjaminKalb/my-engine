use crate::ecs::component::Component;
use crate::ecs::entity::Entity;

pub struct World {
    // Entity storage
    entities: Vec<EntitySlot>,
    free_entities: Vec<u32>,

    // Component storages
}

struct EntitySlot {
    generation: u32,
    alive: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            free_entities: Vec::new(),
        }

    }
    pub fn spawn(&mut self) -> Entity {
        if let Some(id) = self.free_entities.pop() {
            let slot = &mut self.entities[id as usize];

            slot.alive = true;

            return Entity::new(id, slot.generation);
        }

        let id = self.entities.len() as u32;

        self.entities.push(EntitySlot {
           generation: 0,
            alive: true,
        });

        Entity::new(id, 0)
    }
    pub fn despawn(&mut self, entity: Entity) -> bool {
        todo!()
    }
    pub fn add<T: Component>(&mut self, entity: Entity, component: T) {
        todo!()
    }
    pub fn remove<T: Component>(&mut self, entity: Entity) -> Option<T> {
        todo!()
    }
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        todo!()
    }
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        todo!()
    }
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        todo!()
    }
}