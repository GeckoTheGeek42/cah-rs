const ZERO: f32 = 0.0;
const HALF: f32 = 0.5;
const THQU: f32 = 0.75;
const FULL: f32 = 1.0;
pub const LIME	    : RGB = RGB( ZERO,  FULL,  ZERO  );
pub const GREEN 	: RGB = RGB( ZERO,  HALF,  ZERO  );
pub const RED  	    : RGB = RGB( FULL,  ZERO,  ZERO  );
pub const BLUE 	    : RGB = RGB( ZERO,  ZERO,  FULL  );
pub const YELLOW    : RGB = RGB( FULL,  FULL,  ZERO  );
pub const PINK      : RGB = RGB( FULL,  ZERO,  FULL  );
pub const CYAN		: RGB = RGB( ZERO,  FULL,  FULL  );
pub const WHITE     : RGB = RGB( FULL,  FULL,  FULL  );
pub const BLACK		: RGB = RGB( ZERO,  ZERO,  ZERO  );
pub const GRAY		: RGB = RGB( HALF,	HALF,  HALF  );
pub const SILVER	: RGB = RGB( THQU,  THQU,  THQU  );
pub const PURPLE	: RGB = RGB( HALF,  ZERO,  HALF  );
pub const MAROON	: RGB = RGB( HALF,  ZERO,  ZERO  );
pub const TEAL		: RGB = RGB( ZERO,  HALF,  HALF  );
pub const NAVY		: RGB = RGB( ZERO,  ZERO,  HALF  );
pub const ORANGE	: RGB = RGB( FULL,  HALF,  ZERO  );

use std::convert::{From, Into};
use std::ops::Add;
use std::default::Default;
#[derive(Clone, Debug)]
pub struct RGBA(pub f32, pub f32, pub f32, pub f32);
#[derive(Clone, Debug)]
pub struct RGB(pub f32, pub f32, pub f32);
#[derive(Debug)] pub struct Red(pub f32);
#[derive(Debug)] pub struct Blue(pub f32);
#[derive(Debug)] pub struct Green(pub f32);
#[derive(Debug)] pub struct Alpha(pub f32);
impl Default for RGBA {
	fn default() -> RGBA {
		From::from((Red::default(), Green::default(), Blue::default(), Alpha::default()))
	}
}
impl Default for RGB {
	fn default() -> RGB {
		From::from((Red::default(), Green::default(), Blue::default()))
	}
}
impl Default for Alpha {
	#[inline] fn default() -> Alpha {
		Alpha(1.0)
	}
}
impl Default for Red {
	#[inline] fn default() -> Red {
		Red(1.0)
	}
}
impl Default for Green {
	#[inline] fn default() -> Green {
		Green(1.0)
	}
}
impl Default for Blue {
	#[inline] fn default() -> Blue {
		Blue(1.0)
	}
}
impl From<(f32, f32, f32)> for RGB {
	#[inline] fn from((r, g, b): (f32, f32, f32)) -> RGB {
		RGB(r, g, b)
	}
}
impl From<Red> for RGB {
	#[inline] fn from(Red(v): Red) -> RGB {
		RGB(v, 0.0, 0.0)
	}
}
impl From<Green> for RGB {
	#[inline] fn from(Green(v): Green) -> RGB {
		RGB(0.0, v, 0.0)
	}
}
impl From<Blue> for RGB {
	#[inline] fn from(Blue(v): Blue) -> RGB {
		RGB(0.0, 0.0, v)
	}
}
impl From<(Red, Green, Blue)> for RGB {
	#[inline] fn from((Red(r), Green(g), Blue(b)): (Red, Green, Blue)) -> RGB {
		RGB(r, g, b)
	}
}
impl From<RGBA> for RGB {
	#[inline] fn from(RGBA(r, g, b, _): RGBA) -> RGB {
		RGB(r, g, b)
	}
}
impl From<(f32, f32, f32, f32)> for RGBA {
	#[inline] fn from((r, g, b, a): (f32, f32, f32, f32)) -> RGBA {
		RGBA(r, g, b, a)
	}
}
impl From<(Red, Alpha)> for RGBA {
	#[inline] fn from((Red(c), Alpha(a)): (Red, Alpha)) -> RGBA {
		RGBA(c, 0.0, 0.0, a)
	}
}
impl From<(Green, Alpha)> for RGBA {
	#[inline] fn from((Green(c), Alpha(a)): (Green, Alpha)) -> RGBA {
		RGBA(0.0, c, 0.0, a)
	}
}
impl From<(Blue, Alpha)> for RGBA {
	#[inline] fn from((Blue(c), Alpha(a)): (Blue, Alpha)) -> RGBA {
		RGBA(0.0, 0.0, c, a)
	}
}
impl From<(RGB, Alpha)> for RGBA {
	#[inline] fn from((RGB(r, g, b), Alpha(a)): (RGB, Alpha)) -> RGBA {
		RGBA(r, g, b, a)
	}
}
impl From<Red> for RGBA {
	#[inline] fn from(Red(c): Red) -> RGBA {
		RGBA(c, 0.0, 0.0, Alpha::default().0)
	}
}
impl From<Green> for RGBA {
	#[inline] fn from(Green(c): Green) -> RGBA {
		RGBA(0.0, c, 0.0, Alpha::default().0)
	}
}
impl From<Blue> for RGBA {
	#[inline] fn from(Blue(c): Blue) -> RGBA {
		RGBA(0.0, 0.0, c, Alpha::default().0)
	}
}
impl From<RGB> for RGBA {
	#[inline] fn from(RGB(r, g, b): RGB) -> RGBA {
		RGBA(r, g, b, Alpha::default().0)
	}
}
impl From<(Red, Green, Blue, Alpha)> for RGBA {
	#[inline] fn from((Red(r), Green(g), Blue(b), Alpha(a)): (Red, Green, Blue, Alpha)) -> RGBA {
		RGBA(r, g, b, a)
	}
}
impl From<RGBA> for [f32; 4] {
	#[inline] fn from(RGBA(r, g, b, a): RGBA) -> [f32; 4] {
		[r, g, b, a]
	}
}
impl From<RGB> for [f32; 4] {
	#[inline] fn from(RGB(r, g, b): RGB) -> [f32; 4] {
		[r, g, b, Alpha::default().0]
	}
}
impl <C: Into<RGBA>> Add<C> for RGBA {
	type Output = RGBA;
	fn add(self, rhs: C) -> RGBA {
		let RGBA(fr, fg, fb, _) = rhs.into();
		let RGBA(br, bg, bb, ba) = self;
		RGBA((br + fr).min(1.0), (bg + fg).min(1.0), (bb + fb).min(1.0), ba)
	}
}
impl<C: Into<RGB>> Add<C> for RGB {
	type Output = RGB;
	fn add(self, rhs: C) -> RGB {
		let RGB(fr, fg, fb) = rhs.into();
		let RGB(br, bg, bb) = self;
		RGB((br + fr).min(1.0), (bg + fg).min(1.0), (bb + fb).min(1.0))
	}
}
pub trait Mix<FG = Self> {
	type Output;
	fn mix<I: Into<FG>>(self, fg: I) -> Self::Output;
}
impl<B: Into<RGB>> Mix<RGB> for B {
	type Output = RGB;
	fn mix<I: Into<RGB>>(self, fg: I) -> RGB {
		fn ave(f: f32, b: f32) -> f32 { (f + b) / 2.0 }

		let RGB(rb, gb, bb) = self.into();
		let RGB(rf, gf, bf) = fg.into();
		
		RGB( ave(rb, rf), ave(gb, gf), ave(bb, bf) )
	}
}
impl<C: Into<RGBA>> Mix<RGBA> for C {
	type Output = RGBA;
	fn mix<I: Into<RGBA>>(self, fg: I) -> RGBA {
		let RGBA(r_fg, g_fg, b_fg, a_fg) = fg.into();
		let RGBA(r_bg, g_bg, b_bg, a_bg) = self.into();

		let alpha_f = a_bg + a_fg - (a_bg / a_fg);
		if alpha_f < 1.0e-6 { return RGBA(0.0, 0.0, 0.0, 0.0) }

		let (r_bg_a, g_bg_a, b_bg_a) = (r_bg * a_bg, g_bg * a_bg, b_bg * a_bg);
		let (r_fg_a, g_fg_a, b_fg_a) = (r_fg * a_fg, g_fg * a_fg, b_fg * a_fg);

		let red_f   = r_fg_a + r_bg_a * (1.0 - a_fg);
		let green_f = g_fg_a + g_bg_a * (1.0 - a_fg);
		let blue_f  = b_fg_a + b_bg_a * (1.0 - a_fg);

		From::from((red_f / alpha_f, green_f / alpha_f, blue_f / alpha_f, alpha_f))
	}
}