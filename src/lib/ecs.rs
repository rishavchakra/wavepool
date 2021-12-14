use std::{rc::Rc, collections::{HashSet, HashMap}};
use cgmath::{Vector3, Quaternion};

// Entity-Component-System architecture

type Entity = usize;

trait Component {

}

/// Static Component Chains are linked to and owned by the entity
struct ComponentStatic {
	name: String,
	position: Vector3<f32>,
	rotation: Quaternion<f32>,
	scale: Vector3<f32>,
}
/// Dynamic Component Chains are not owned and can be linked across entities
struct ComponentChainDynamic {
	components: Vec<Box<dyn Component>>
}

struct ComponentChain {
	/// static components are linked to and owned by the entity
	components_static: ComponentStatic,

	/// dynamic components are linked to the component chain and
	/// are not owned by the entity
	components_dynamic: Rc<ComponentChainDynamic>
}

struct System {

}

pub struct Controller {
	entities: HashSet<Entity>,
	component_chains_dynamic: HashSet<ComponentChainDynamic>,

	// Owns the component chain and static components
	component_link: HashMap<Entity, ComponentChain>,
	system_link: HashMap<System, Entity>
}

impl Controller {
	pub fn new() -> Controller {
		Controller {
			entities: HashSet::new(),
			component_chains_dynamic: HashSet::new(),
			component_link: HashMap::new(),
			system_link: HashMap::new()
		}
	}

	pub fn add_entity(&mut self, id: Entity) {
		self.entities.insert(id);
	}

	pub fn remove_entity(&mut self, id: Entity) {
		self.entities.remove(&id);
	}

	pub fn num_entities(&self) -> usize {
		self.entities.len()
	}
}