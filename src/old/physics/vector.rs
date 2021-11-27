use std::ops::{Add, Sub};

pub struct vec2d {
	pub x: f32,
	pub y: f32
}

impl Add for vec2d {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Sub for vec2d {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y
		}
	}
}
