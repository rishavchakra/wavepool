use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use super::component;
use super::entity;
/**
 * System in the ECS Entity Component System
 * Connects the Entities to their components
 */

pub struct System {
    // Container for the system's component chains
    components: HashSet<Rc<ComponentChain>>,
    // Connection between the Entities and their Component chains
    entity_link: HashMap<Entity, Rc<ComponentChain>>,
}

struct ComponentChain {
    components: Vec<Component>,
}