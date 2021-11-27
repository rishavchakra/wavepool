use super::{physMaster::physObject, vector::vec2d};

pub struct sbObject {
	nodes: Vec<sbNode>
}

struct sbNode {
	mass: f32,
	pos: vec2d,
	vel: vec2d,
	acc: vec2d
}

impl sbObject {
	fn calculateForces(&self) {

	}

	fn moveNodes(&mut self) {

	}
}

impl physObject for sbObject {
	fn step(&mut self, dt: f32) {
		self.calculateForces();
		self.moveNodes();
	}
}