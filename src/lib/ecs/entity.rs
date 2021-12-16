use std::collections::{VecDeque, HashSet};

pub type Entity = usize;

pub struct EntityManager {
    entities: HashSet<Entity>,
    max_entities: usize
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entities: HashSet::new(),
            max_entities: 0
        }
    }

    pub fn createEntity(&mut self) -> Entity {
        let id = self.max_entities; // Should result in a unique entity id
        self.entities.insert(id); // TODO: Make this unique
        self.max_entities += 1; // Increments for the next entity created
        id
    }

    pub fn destroyEntity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    pub fn numEntities(&self) -> usize {
        self.entities.len()
    }
}