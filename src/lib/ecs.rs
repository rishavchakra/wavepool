mod component;
mod entity;
mod system;
use component::ComponentManager;
use entity::{Entity, EntityManager};
use system::SystemManager;

use self::component::ComponentDynamic;

// Entity-Component-System architecture

pub struct Controller {
	entity_manager: EntityManager,
	component_manager: ComponentManager,
	system_manager: SystemManager
}

impl Controller {
    pub fn new() -> Controller {
        todo!()
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
	pub fn create_component(&self, entity: Entity, component: ComponentDynamic) {
		self.component_manager.create_component(entity, component)
	}


	// System functions
}
