use std::{
    collections::HashMap, rc::{Rc, Weak}
};

use cgmath::{Quaternion, Vector3};

use super::Entity;

pub enum Component {
    Name(String),
    Transform(TransformCompData),
    Gravity(GravityCompData),
    Collision(CollisionCompData), // Collision mesh
    Script(ScriptCompData),
    // ComponentChain(CompChainCompData) This could pose problems with reflexive linking, figure it out later
}

#[derive(PartialEq, Eq, Hash)]
pub enum ComponentType {
    Name,
    Transform,
    Gravity,
    Collision,
    Script,
    None
}

impl Component {

    /**
     * There has to be a better way of doing this. It works, but it's a little hacky
     * TODO: Try to figure something out here.
     */
    pub fn get_type(&self) -> ComponentType {
        match self {
            Name => ComponentType::Name,
            Transform => ComponentType::Transform,
            Gravity => ComponentType::Gravity,
            Collision => ComponentType::Collision,
            Script => ComponentType::Script,
            _ => ComponentType::None
        }
    }
}

/* * * * * Component Chain Structs * * * * */
pub struct ComponentChain {
    component_chain: Vec<Component>,
}

/* * * * * Component Chain Implementations * * * * */
impl ComponentChain {
    pub fn from_component(component: Component) -> Self {
        Self {
            component_chain: vec![component],
        }
    }

    pub fn insert(&mut self, component: Component) {
        self.component_chain.push(component)
    }
}

/* * * * * Component Manager Struct * * * * */
pub struct ComponentManager {
    component_link: HashMap<Entity, ComponentChain>,
    entity_link: HashMap<ComponentType, Vec<Entity>>,
}

/* * * * * Component Manager Implementation * * * * */
impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_link: HashMap::new(),
            entity_link: HashMap::new(),
        }
    } // fn new

    /**
     * Creates a default component of the given type
     */
    pub fn create_component(&mut self, entity: Entity, component_type: ComponentType) {

    }

    /**
     * Adds a component to an existing entity's component chain.
     * If the entity doesn't have a component chain, creates a chain and adds the component to it.
     */
    pub fn register_component(&mut self, entity: Entity, component: Component) {
        // Add to the components' connected entities
        if let Some(entities) = self.entity_link.get_mut(&component.get_type()) {
            entities.push(entity.clone());
        } else {
            self.entity_link.insert(component.get_type(), vec![entity]);
        }

        // Add to the entities' connection to the components
        if let Some(component_chain) = self.component_link.get_mut(&entity) {
            component_chain.insert(component);
        } else {
            self.component_link
                .insert(entity, ComponentChain::from_component(component));
        }
    } // fn create_component

    pub fn get_entities_of_component(&self, component_type: ComponentType) -> Rc<Vec<Entity>> {
        todo!()
    }

    pub fn get_components_of_entity(&self, entity: Entity) -> Option<Rc<ComponentChain>> {
        match self.component_link.get(&entity) {
            Some(component_chain) => {
                // Some(Rc::from(component_chain))
                None // TODO: Figure out how to fix this
            }
            None => None
        }
    }
}

/* * * * * Component Data Structs * * * * */
pub struct TransformCompData {
    pos_vec: Vector3<f32>,
    rot_quat: Quaternion<f32>,
    scale_vec: Vector3<f32>,
}

impl Default for TransformCompData {
    fn default() -> Self {
        Self {
            pos_vec: Vector3::new(0., 0., 0.),
            rot_quat: Quaternion::new(0., 0., 0., 0.),
            scale_vec: Vector3::new(1., 1., 1.),
        }
    }
} // Default for Transform Component Data

pub struct GravityCompData {
    g_vec: Vector3<f32>,
}

impl Default for GravityCompData {
    fn default() -> Self {
        Self {
            g_vec: Vector3::new(0., 0., -9.8),
        }
    }
} // Default for Gravity Component Data

pub struct CollisionCompData {
    // Collision mesh
}

#[derive(Default)]
pub struct ScriptCompData {
    path: String,
}

/// Deprecated, for now
pub struct CompChainCompData {
    // some kind of pointer/reference to a Component Chain
}
