mod ecs;

#[cfg(test)]
mod tests {
    use crate::ecs::{self, component::ComponentType};

	#[test]
	pub fn test_ecs_entities_add_remove() {
		let mut ecs_controller = ecs::Controller::new();

		let entity = ecs_controller.create_entity().unwrap();
		assert_eq!(ecs_controller.num_entities(), 1);

		ecs_controller.destroy_entity(entity);
		assert_eq!(ecs_controller.num_entities(), 0);
	}

	pub fn test_ecs_components_add_remove() {
		let mut ecs_controller = ecs::Controller::new();

		/*
		let Some(entity) = ecs_controller.create_entity();
		assert_eq!(ecs_controller.get_entities_of_component(ComponentType::Name), Vec::new()); // Change this test if/when entity creation has default components

		ecs_controller.create_component(entity, ComponentType::Gravity);
		assert_eq!(ecs_controller.get_entities_of_component(ComponentType::Gravity), vec![entity]);

		ecs_controller.create_component(entity, ComponentType::None);
		*/
	}

	// #[test]
	pub fn test() {

	}
}