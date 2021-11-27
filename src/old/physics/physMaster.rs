pub trait physObject {
	fn step(&mut self, dt: f32);
}

pub struct physMaster {
	objects: Vec<Box<dyn physObject>>
}

impl physMaster {
	/// Instructs all the physics objects to step according to the tick time
	pub fn stepAll(&mut self, dt: f32) {
		for object in &mut self.objects {
			object.step(dt);
		}
	}
}