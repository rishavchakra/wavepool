use std::collections::{HashSet};

pub type Entity = usize;
const ENTITY_LIMIT: usize = 5000;

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

    pub fn create_entity(&mut self) -> Option<Entity> {
        if self.entities.len() >= ENTITY_LIMIT {
            return None
        }

        let id = self.max_entities; // Should result in up to 2^32 unique entity ids
        self.entities.insert(id);
        self.max_entities += 1; // Increments for the next entity created
        Some(id)
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }
}