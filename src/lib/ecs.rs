pub mod component;
mod entity;
mod system;
use std::rc::{Rc, Weak};

use component::ComponentManager;
use entity::{Entity, EntityManager};
use system::SystemManager;

use self::component::{Component, ComponentType, ComponentChain};

// Entity-Component-System architecture

pub struct Controller {
	entity_manager: EntityManager,
	component_manager: ComponentManager,
	system_manager: SystemManager
}

impl Controller {
    pub fn new() -> Controller {
		Controller {
			entity_manager: EntityManager::new(),
			component_manager: ComponentManager::new(),
			system_manager: SystemManager::new()
		}
    }

	// Entity functions
	pub fn create_entity(&mut self) -> Option<Entity> {
		self.entity_manager.create_entity()
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		self.entity_manager.destroy_entity(entity)
	}

	pub fn num_entities(&self) -> usize {
		self.entity_manager.num_entities()
	}

	// Component functions
	pub fn create_component(&mut self, entity: Entity, component_type: ComponentType) {
		todo!()
	}

	pub fn register_component(&mut self, entity: Entity, component: Component) {
		self.component_manager.register_component(entity, component)
	}

	pub fn get_entities_of_component(&self, component_type: ComponentType) -> Rc<Vec<Entity>> {
		self.component_manager.get_entities_of_component(component_type)
	}

	pub fn get_components_of_entity(&self, entity: Entity) -> Option<Rc<ComponentChain>> {
		self.component_manager.get_components_of_entity(entity)
	}


	// System functions
}
