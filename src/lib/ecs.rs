mod component;
mod entity;
mod system;
use component::{ComponentDynamicChain, ComponentManager, ComponentStaticChain};
use entity::{Entity, EntityManager};
use system::{SystemManager};

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
	pub fn createEntity(&self) -> Entity {
		self.entity_manager.createEntity()
	}

	pub fn destroyEntity(&self, entity: Entity) {
		self.entity_manager.destroyEntity(entity)
	}

	// Component functions


	// System functions
}
