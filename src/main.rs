extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use std::f64::consts::PI;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

static DEPTH:u32 = 21;
static WIDTH:u32 = 15;

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
			Button::Keyboard(Key::Up) => {
				self.grid.rotate_cursor()
			}
			Button::Keyboard(Key::Right) => {
				self.grid.move_right();
			}
			Button::Keyboard(Key::Left) => {
				self.grid.move_left();
			}
			Button::Keyboard(Key::Down) => {
				self.grid.move_down();
			},
			Button::Keyboard(Key::Space) => {
				self.grid.drop();
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
struct Cell { x: u32, y: u32 }
struct Shape {
	x: u32, y: u32,
	cells: Vec<Vec<Cell>>,
	orientation: usize,
	colour: [f32; 4]
}
impl Shape {
	fn current_shape(&self) -> &Vec<Cell> {
		&self.cells[self.orientation]
	}
	fn rotate(&mut self) {
		self.orientation = (self.orientation + 1) % self.cells.len();
	}
	fn remove_row(&mut self, row: u32) {
		println!("Remove row from shape");
		// remove cells
		let mut removed_any;
		loop {
			removed_any = false;
			for i in 0..self.current_shape().len() {
				if self.current_shape()[i].y + self.y == row {
					self.cells[self.orientation].remove(i);
					removed_any = true;
					break;
				}
			}
			if !removed_any { break; }
		}
		// lower all cells above row
		self.y += 1;
	}
	fn lowest_y(&self) -> u32 {
		let mut y = 0;
		for cell in self.current_shape() {
			if cell.y > y {
				y = cell.y;
			}
		}
		y + self.y
	}
	fn collides(&self, shape: &Shape) -> bool {
		let current_shape = self.current_shape();
		let other_current_shape = shape.current_shape();
		for cell in current_shape {
			let x1 = cell.x + self.x;
			let y1 = cell.y + self.y;
			if y1 > DEPTH {
				return true
			}
			for other_cell in other_current_shape {
				let x2 = other_cell.x + shape.x;
				let y2 = other_cell.y + shape.y;
				if x1 == x2 && y1 == y2 {
					return true
				}
			}
		}
		false
	}
}
struct Grid {
	cursor: usize,
	shapes: Vec<Shape>
}
impl Grid {
	fn drop(&mut self) {
		loop {
			if !self.move_down() {
				break;
			}
		}
	}
	fn rotate_cursor(&mut self) {
		self.shapes[self.cursor].rotate()
	}
	fn clashes(&self, shape: &Shape) -> bool {
		for i in 0..self.shapes.len() {
			if i != self.cursor && shape.collides(&self.shapes[i]) {
				return true
			}
		}
		shape.lowest_y() > DEPTH || false
	}
	fn move_right(&mut self) {
		if self.shapes[self.cursor].x < WIDTH {
			self.shapes[self.cursor].x += 1;
			if self.clashes(&self.shapes[self.cursor]) {
				self.shapes[self.cursor].x -= 1;
			}
		}
	}
	fn move_left(&mut self) {
		if self.shapes[self.cursor].x > 0 {
			self.shapes[self.cursor].x -= 1;
			if self.clashes(&self.shapes[self.cursor]) {
				self.shapes[self.cursor].x += 1;
			}
		}
	}
	fn move_down(&mut self) -> bool {
		self.shapes[self.cursor].y += 1;
		if self.clashes(&self.shapes[self.cursor]) {
			self.shapes[self.cursor].y -= 1;
			self.handle_placed();
			false
		} else { true }
	}
	fn handle_placed(&mut self) {
		self.shapes.push(make_shape());
		self.cursor = self.shapes.len()-1;
		if let Some(mut row) = self.get_row_if_full() {
			for shape in &mut self.shapes {
				shape.remove_row(row);
			}
		}
	}
	fn get_row_if_full(&mut self) -> Option<u32> {
		for j in 0..DEPTH+1 {
			print!("{} ", j);
			let mut row_full = true;
			for i in 0..WIDTH+3 {
				let mut cell_full = false;
				for shape in &self.shapes {
					for cell in shape.current_shape() {
						if shape.x + cell.x == i && shape.y + cell.y == j {
							cell_full = true;
							break;
						}
					}
				}
				print!("{} ", if cell_full { "X" } else { "_" });
				row_full = row_full && cell_full;
			}
			println!("");
			if row_full {
				return Some(j);
			}
		}
		None
	}
	fn draw(
		&mut self,
		c: graphics::Context,
		gl: &mut GlGraphics,
	) {
		use graphics::*;
		for mut shape in &self.shapes {
			//shape.draw(c, gl)
			let current_shape = shape.current_shape();
			for cell in current_shape {
				let square = rectangle::square(0.0, 0.0, 40.0);
				let transform = c.transform.trans(
					((shape.x + cell.x) * 40) as f64,
					((shape.y + cell.y) * 40) as f64
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

fn make_shape() -> Shape {
	use rand::Rng;
	let mut rng = rand::thread_rng();
	if rng.gen() {
		shape1(5, 0)
	} else {
		shape2(5, 0)
	}
}

fn shape1(x: u32, y: u32) -> Shape {
	Shape {
		x: x, y: y,
		orientation: 0,
		cells: vec![
			vec![
				Cell { x: 0, y: 1 },
				Cell { x: 1, y: 1 },
				Cell { x: 2, y: 1 },
				Cell { x: 1, y: 2 }
			],
			vec![
				Cell { x: 1, y: 0 },
				Cell { x: 0, y: 1 },
				Cell { x: 1, y: 1 },
				Cell { x: 1, y: 2 }
			],
			vec![
				Cell { x: 0, y: 1 },
				Cell { x: 1, y: 1 },
				Cell { x: 1, y: 0 },
				Cell { x: 2, y: 1 }
			],
			vec![
				Cell { x: 1, y: 0 },
				Cell { x: 1, y: 1 },
				Cell { x: 2, y: 1 },
				Cell { x: 1, y: 2 }
			]
		],
		colour: [1.0, 0.0, 0.0, 1.0]
	}
}
fn shape2(x: u32, y: u32) -> Shape {
	Shape {
		x: x, y: y,
		orientation: 0,
		cells: vec![
			vec![
				Cell { x: 0, y: 0 },
				Cell { x: 1, y: 0 },
				Cell { x: 1, y: 1 },
				Cell { x: 2, y: 1 }
			],
			vec![
				Cell { x: 2, y: 0 },
				Cell { x: 1, y: 1 },
				Cell { x: 2, y: 1 },
				Cell { x: 1, y: 2 }
			],
		],
		colour: [0.0, 0.0, 1.0, 1.0]
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

	let cursor = shape1(5,0);
	let mut app = App {
		gl: GlGraphics::new(opengl),
		background: Background { colour: [0.0, 1.0, 0.0, 1.0] },
		grid: Grid {
			cursor: 0,
			shapes: vec![
				cursor,
			]
		}
	};

	let mut events = window.events();
	let mut timer = 0.0;
	while let Some(e) = events.next(&mut window) {
		if let Some(r) = e.render_args() {
			app.render(&r);
		}
		if let Some(u) = e.update_args() {
			let new_time = (timer + 4.0*u.dt) % 1.0;
			if new_time < timer {
				app.grid.move_down();
			}
			timer = new_time;
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
