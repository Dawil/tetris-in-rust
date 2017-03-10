extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::f64::consts::PI;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

struct App {
	gl: GlGraphics,
	background: Background,
	grid: Grid
}
impl App {
	fn render(&mut self, args: &RenderArgs) {
		let ref mut background = self.background;
		let ref mut grid = self.grid;
		self.gl.draw(args.viewport(), |c, gl| {
			background.draw(gl);
			grid.draw(c, gl);
		});
	}
	fn update(&mut self, args: &UpdateArgs) {
	}
	fn user_input(&mut self, button: &Button) {
		match *button {
			Button::Keyboard(Key::Space) => {
				self.grid.shapes[0].rotate();
			}
			Button::Keyboard(Key::Right) => {
				self.grid.shapes[0].move_right();
			}
			Button::Keyboard(Key::Left) => {
				self.grid.shapes[0].move_left();
			}
		//	Button::Keyboard(Key::Up) => {
		//		self.up_d = true;
		//	}
			Button::Keyboard(Key::Down) => {
				self.grid.shapes[0].move_down();
			},
			_ => {}
		}
	}
	fn user_release(&mut self, button: &Button) {
	}
}
struct Background {
	colour: [f32; 4]
}
struct Cell { x: usize, y: usize }
impl Cell {
	fn dec_y(&mut self) { self.y -= 1; }
}
struct Shape {
	cells: Vec<Cell>,
	colour: [f32; 4]
}
impl Shape {
	fn top_left_cell(&self) -> Cell {
		let mut x = 1000;
		let mut y = 1000;
		for cell in &self.cells {
			if cell.x < x { x = cell.x; }
			if cell.y < y { y = cell.y; }
		}
		Cell { x: x, y: y }
	}
	fn rotate(&mut self) {
		let top_left_cell = self.top_left_cell();
		for cell in &mut self.cells {
			// transpose cell (matrix multiplcation)
		}
	}
	fn move_left(&mut self) {
		for cell in &mut self.cells {
			cell.x -= 1;
		}
	}
	fn move_right(&mut self) {
		for cell in &mut self.cells {
			cell.x += 1;
		}
	}
	fn move_down(&mut self) {
		for cell in &mut self.cells {
			cell.y += 1;
		}
	}
	fn draw(
		&mut self,
		c: graphics::Context,
		gl: &mut GlGraphics,
	) {
		use graphics::*;
		for cell in self.cells.iter() {
			let square = rectangle::square(0.0, 0.0, 40.0);
			let transform = c.transform.trans(
				cell.x as f64,
				cell.y as f64
			);
			rectangle(self.colour, square, transform, gl);
		}
	}
}
struct Grid {
	shapes: Vec<Shape>
}
impl Grid {
	fn draw(
		&mut self,
		c: graphics::Context,
		gl: &mut GlGraphics,
	) {
		use graphics::*;
		for mut shape in &self.shapes {
			//shape.draw(c, gl)
			for cell in &shape.cells {
				let square = rectangle::square(0.0, 0.0, 40.0);
				let transform = c.transform.trans(
					(cell.x * 40) as f64,
					(cell.y * 40) as f64
				);
				rectangle(shape.colour, square, transform, gl);
			}
		}
	}
}

impl Background {
	fn draw(&mut self, gl: &mut GlGraphics) {
		use graphics::*;
		clear(self.colour, gl);
	}
}

fn main() {
	let opengl = OpenGL::V3_2;

	let mut window: Window = WindowSettings::new(
		"program",
		[200,200]
	).opengl(opengl)
	.build()
	.unwrap();

	let mut app = App {
		gl: GlGraphics::new(opengl),
		background: Background { colour: [0.0, 1.0, 0.0, 1.0] },
		grid: Grid {
			shapes: vec![
				Shape {
					cells: vec![
						Cell { x: 0, y: 0 },
						Cell { x: 1, y: 0 },
						Cell { x: 2, y: 0 },
						Cell { x: 1, y: 1 }
					],
					colour: [1.0, 0.0, 0.0, 1.0]
				}
			]
		}
	};

	let mut events = window.events();
	while let Some(e) = events.next(&mut window) {
		if let Some(r) = e.render_args() {
			app.render(&r);
		}
		if let Some(u) = e.update_args() {
			app.update(&u);
		}
		if let Some(p) = e.press_args() {
			app.user_input(&p);
		}
		if let Some(p) = e.release_args() {
			app.user_release(&p);
		}
	}
}
