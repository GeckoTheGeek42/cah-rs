use piston::window::{ WindowSettings, Size };
use piston::event::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL, glyph_cache };
use std::path::Path;
use piston::input::{Button, MouseButton, Key};
use std::borrow::ToOwned;

pub mod color;
pub mod render;
pub mod viewport;

pub struct AppWindow<'a>(App<'a>, Window);

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    character_cache: glyph_cache::GlyphCache<'a>,
    state: AppState,
}

impl<'a> AppWindow<'a> {
	pub fn display(mut self) {
		let AppWindow(mut app, mut window) = self;
		for e in window.events() {
			if let Some(r) = e.render_args() {
			    app.render(&r);
			}

			if let Some(u) = e.update_args() {
			    app.update(&u);
			}

			if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
				app.state.clicked = true;
			}
			if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
				app.state.clicked = false;
				app.state.worked = false;
			}
		}
	}
}

impl<'a> App<'a> {
	pub fn new() -> AppWindow<'a> {
		let gl = OpenGL::_3_2;
		let window = Window::new(
			gl,
			WindowSettings::new(
				"Cards Against Humanity".to_owned(), 
				Size { width: 1280, height: 720 }
			).exit_on_esc(true)
		);
		let app = App {
			gl: GlGraphics::new(gl),
	        character_cache: glyph_cache::GlyphCache::new(&Path::new("C:\\windows\\fonts\\arial.ttf")).unwrap(),
			state: AppState::new(),
		};
		AppWindow(app, window)
	}

	fn render(&mut self, args: &RenderArgs) {
    	use graphics::*;
    	use self::color::*;

    	//Setup
        let context = &Context::abs(args.width as f64, args.height as f64);
        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2) as f64);
        let mouse_context = &context.trans(self.state.mouse_cursor.0, self.state.mouse_cursor.1);

        fn shrink_rect(rect: &types::Rectangle, factor: f64) -> types::Rectangle {
        	let factor2 = factor * 2.0;
        	[ rect[0] + factor, rect[1] + factor, rect[2] - factor2, rect[3] - factor2 ]
        }

    	self.gl.draw(viewport::viewport(args.width, args.height), |_, gl| {
    		clear(WHITE.into(), gl);
    	})
	}

	fn update(&mut self, args: &UpdateArgs) {
		if self.state.clicked && !self.state.worked {
			self.state.worked = true;
		}
	}
}

pub struct AppState {
	mouse_cursor: (f64, f64),
	// button: Option<Button>,
	clicked: bool, worked: bool
}

impl AppState {
	fn new() -> AppState {
		AppState {
			mouse_cursor: (0.0, 0.0),
			// button: None,
			clicked: false, worked: false
		}
	}
}