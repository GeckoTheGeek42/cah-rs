use graphics::Viewport;

// use std::default::Default;
// const DEFAULT_SIZE: (u32, u32) = (1280, 720);

pub fn viewport(width: u32, height: u32) -> Viewport {
	Viewport {
	   	rect: [0, 0, width as i32, height as i32],
	   	draw_size: [width as u32, height as u32],
	   	window_size: [width as u32, height as u32],
	}
}

#[derive(Clone, Debug)] pub struct VPRect(i32, i32, i32, i32);

impl From<(i32, i32, i32, i32)> for VPRect {
	#[inline]
	fn from((x, y, w, h): (i32, i32, i32, i32)) -> VPRect {
		VPRect(x, y, w, h)
	}
}

impl From<(i32, i32)> for VPRect {
	#[inline]
	fn from((w, h): (i32, i32)) -> VPRect {
		VPRect(0, 0, w, h)
	}
}

#[derive(Clone, Debug)] pub struct VPDrawSize(u32, u32);

impl From<(u32, u32)> for VPDrawSize {
	#[inline]
	fn from((x, y): (u32, u32)) -> VPDrawSize {
		VPDrawSize(x, y)
	}
}

#[derive(Clone, Debug)] pub struct VPWindowSize(u32, u32);

impl From<(u32, u32)> for VPWindowSize {
	#[inline]
	fn from((x, y): (u32, u32)) -> VPWindowSize {
		VPWindowSize(x, y)
	}
}

#[derive(Clone, Debug)] pub struct ViewportInfo(pub VPRect, pub VPDrawSize, pub VPWindowSize);

impl ViewportInfo {
	fn new(width: u32, height: u32) -> ViewportInfo {
		ViewportInfo( 
			VPRect(0, 0, width as i32, height as i32), 
			VPDrawSize(width, height), 
			VPWindowSize(width, height) 
		)
	}

	fn rect<I: Into<VPRect>>(mut self, t: I) -> ViewportInfo {
		self.0 = t.into();
		self
	}

	fn with_rect(mut self, x: i32, y: i32, w: i32, h: i32) -> ViewportInfo {
		self.0 = VPRect(x, y, w, h);
		self
	}

	fn draw_size<I: Into<VPDrawSize>>(mut self, t: I) -> ViewportInfo {
		self.1 = t.into();
		self
	}

	fn with_draw_size(mut self, x: u32, y: u32) -> ViewportInfo {
		self.1 = VPDrawSize(x, y);
		self
	}

	fn window_size<I: Into<VPWindowSize>>(mut self, t: I) -> ViewportInfo {
		self.2 = t.into();
		self
	}

	fn with_window_size(mut self, x: u32, y: u32) -> ViewportInfo {
		self.2 = VPWindowSize(x, y);
		self
	}
}

// impl From<(u32, u32)> for ViewportInfo {
// 	fn from((width, height): (u32, u32)) -> ViewportInfo {
// 		ViewportInfo( 
// 			VPRect(0, 0, width as i32, height as i32), 
// 			VPDrawSize(width, height), 
// 			VPWindowSize(width, height) 
// 		)
// 	}
// }

impl From<ViewportInfo> for Viewport {
	#[inline]
	fn from(
		ViewportInfo(
			VPRect(x, y, w, h),
			VPDrawSize(dx, dy),
			VPWindowSize(wx, wy)
		): ViewportInfo
	) -> Viewport {
		Viewport {
			rect: [x, y, w, h],
			draw_size: [dx, dy],
			window_size: [wx, wy]
		}
	}
}	

// pub trait Builder<T> {
// 	fn with<I: Into<T>>(mut self, t: I) -> Self;
// }
// impl Builder<VPRect> for ViewportInfo {
// 	fn with<I: Into<VPRect>>(mut self, t: I) -> ViewportInfo {
// 		self.0 = t.into();
// 		self
// 	}
// }
// impl Builder<VPDrawSize> for ViewportInfo {
// 	fn with<I: Into<VPDrawSize>>(mut self, t: I) -> ViewportInfo {
// 		self.1 = t.into();
// 		self
// 	}
// }
// impl Builder<VPWindowSize> for ViewportInfo {
// 	fn with<I: Into<VPWindowSize>>(mut self, t: I) -> ViewportInfo {
// 		self.2 = t.into();
// 		self
// 	}
// }