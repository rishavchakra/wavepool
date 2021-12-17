use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use cgmath::{Quaternion, Vector3};

use super::Entity;

pub enum Component {
    Dynamic(ComponentDynamic),
    Static(ComponentStatic),
}

/**
 * Dynamic components aren't 'owned' by the entities,
 * and can thus be linked to other entities dynamically
 */
pub enum ComponentDynamic {
    Gravity(GravityCompData),
    Collision(CollisionCompData), // Collision mesh
    Scripts(ScriptCompData),
    // ComponentChain(CompChainCompData) This could pose problems with reflexive linking, figure it out later
}

/**
 * Static components are 'owned' by the entities,
 * and are unique to the entity
 */
pub enum ComponentStatic {
    Name(String),
    Transform(TransformCompData),
}

/* * * * * Component Chain Structs * * * * */
pub struct ComponentStaticChain {
    components: Vec<ComponentStatic>,
}

pub struct ComponentDynamicChain {
    components: Vec<ComponentDynamic>,
}

/* * * * * Component Chain Implementations * * * * */
impl ComponentStaticChain {
    pub fn insert(&mut self, component: ComponentStatic) {
        self.components.push(component);
    }
}

impl ComponentDynamicChain {
    pub fn insert(&mut self, component: ComponentDynamic) {
        self.components.push(component);
    }
}

/* * * * * Component Manager Struct * * * * */
pub struct ComponentManager {
    /// Owned set of component chains that can be used and linked from
    component_chains: HashSet<ComponentDynamicChain>,
    /// Links the entities to their associated dynamic component chains
    component_link_dynamic: HashMap<Entity, Rc<RefCell<ComponentDynamicChain>>>,
    /// Links the entities to their associated static component chains
    component_link_static: HashMap<Entity, Rc<RefCell<ComponentStaticChain>>>,
    /// Links the components to the entities that contain them
    entity_link: HashMap<Rc<Component>, Vec<Entity>>,
}

/* * * * * Component Manager Implementation * * * * */
impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_chains: HashSet::new(),
            component_link_dynamic: HashMap::new(),
            component_link_static: HashMap::new(),
            entity_link: HashMap::new(),
        }
    } // fn new

    /**
     * Adds a component to the entity's component chain
     */
    pub fn create_component(&mut self, entity: Entity, component: Component) {
        match component {
            Component::Dynamic(component_dynamic) => {
                self.component_link_dynamic
                    .get(&entity)
                    .unwrap()
                    .borrow_mut()
                    .insert(component_dynamic);
            }

            Component::Static(component_static) => {
                self.component_link_static
                    .get(&entity)
                    .unwrap()
                    .borrow_mut()
                    .insert(component_static);
            }
        } // match component type
    } // fn create_component
} // impl ComponentManager

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
}

pub struct GravityCompData {
    g_vec: Vector3<f32>,
}

impl Default for GravityCompData {
    fn default() -> Self {
        Self {
            g_vec: Vector3::new(0., 0., -9.8),
        }
    }
}

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
