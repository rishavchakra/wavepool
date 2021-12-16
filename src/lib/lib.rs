mod ecs;

#[cfg(test)]
mod tests {
    use crate::ecs;

	#[test]
	pub fn test_ecs_controller_add_remove() {
		let mut ecs_controller = ecs::Controller::new();

		ecs_controller.create_entity();
		assert_eq!(ecs_controller.num_entities(), 1);

		ecs_controller.create_entity();
		assert_eq!(ecs_controller.num_entities(), 0);
	}

	// #[test]
	pub fn test() {

	}
}