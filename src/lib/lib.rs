mod ecs;

#[cfg(test)]
mod tests {
    use crate::ecs;

	#[test]
	pub fn test_ecs_controller_add_remove() {
		let mut ecs_controller = ecs::Controller::new();

		ecs_controller.createEntity();
		assert_eq!(ecs_controller.numEntities(), 1);

		ecs_controller.createEntity();
		assert_eq!(ecs_controller.numEntities(), 0);
	}

	// #[test]
	pub fn test() {

	}
}