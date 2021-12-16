use std::collections::HashSet;

use cgmath::{Vector3, Quaternion};

use super::{Entity};

/**
 * Dynamic components aren't 'owned' by the entities,
 * and can thus be linked to other entities dynamically
 */
pub enum ComponentDynamic {
    Gravity(GravityCompData),
    Collision(CollisionCompData), // Collision mesh
    Scripts(ScriptCompData),
    ComponentChain(CompChainCompData)
}

/**
 * Static components are 'owned' by the entities,
 * and are unique to the entity
 */
pub enum ComponentStatic {
    Name (String),
    Transform(TransformCompData)
}

pub struct ComponentStaticChain {
    components: Vec<ComponentStatic>
}

pub struct ComponentDynamicChain {
    components: Vec<ComponentDynamic>
}

pub struct ComponentManager {
    component_chains: HashSet<ComponentDynamicChain>
}

impl ComponentManager {
    pub fn create_component(&self, entity: Entity, component: ComponentDynamic) {
        todo!()
    }

}


/* Component Data Structs */
pub struct TransformCompData {
    pos_vec: Vector3<f32>,
    rot_quat: Quaternion<f32>,
    scale_vec: Vector3<f32>
}

pub struct GravityCompData {
    g_vec: Vector3<f32>
}

pub struct CollisionCompData {
    // Collision mesh
}

pub struct ScriptCompData {
    path: String
}

pub struct CompChainCompData {
    // some kind of pointer/reference to a Component Chain
}