use vect::prelude::*;

pub trait Vector2Ext {
	fn length(&self) -> f64;
}

impl Vector2Ext for Vector2 {
	fn length(&self) -> f64 {
		self.magnitude()
	}
}
