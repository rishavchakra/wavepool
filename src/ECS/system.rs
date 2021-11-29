use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use super::component::Component;
use super::entity::Entity;
/**
 * System in the ECS Entity Component System
 * Connects the Entities to their components
 */

pub struct System {
    // Container for the system's entities
    // Owns the entities
    entities: HashSet<Entity>,
    // Container for the system's component chains
    // Owns the components
    components: HashSet<ComponentChain>,

    // Connection between the Entities and their Component chains
    entity_link: HashMap<Entity, Arc<ComponentChain>>,
}

struct ComponentChain {
    components: Vec<Component>,
}

impl System {
    // Constructs a new system with entities and components
    pub fn new() -> System {
        System {
            entities: HashSet::new(),
            components: HashSet::new(),
            entity_link: HashMap::new()
        }
    }

    pub fn add_entity(&mut self) {
        let entity = Entity::new();
        self.entities.insert(entity);
    }
}
