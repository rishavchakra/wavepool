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
	pub fn createEntity(&mut self) -> Entity {
		self.entity_manager.createEntity()
	}

	pub fn destroyEntity(&mut self, entity: Entity) {
		self.entity_manager.destroyEntity(entity)
	}

	pub fn numEntities(&self) -> usize {
		self.entity_manager.numEntities()
	}

	// Component functions


	// System functions
}
