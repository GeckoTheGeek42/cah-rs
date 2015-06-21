use graphics::math::Matrix2d;
use graphics::{Graphics, DrawState};

pub trait Render {
	fn draw<G: Graphics>(&self, gl: &mut G, transform: Matrix2d, draw_state: DrawState);
}

impl<'a, R: Render> Render for &'a [R] {
	fn draw<G: Graphics>(&self, gl: &mut G, transform: Matrix2d, draw_state: DrawState) {
		self.iter().map(|r| r.draw(gl, transform, draw_state)).collect::<Vec<_>>();
	}
}