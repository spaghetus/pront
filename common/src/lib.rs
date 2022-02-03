use std::ops::{Add, Mul};

use rusttype::OutlineBuilder;

#[cfg(feature = "f64")]
pub type F = f64;
#[cfg(feature = "f32")]
pub type f = f32;

#[derive(Default, Debug)]
pub struct Vec2 {
	pub x: F,
	pub y: F,
}

impl Mul<F> for Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: F) -> Vec2 {
		Vec2 {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl Add<Vec2> for Vec2 {
	type Output = Vec2;
	fn add(self, rhs: Vec2) -> Vec2 {
		Vec2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

#[non_exhaustive]
#[derive(Default, Debug)]
pub struct TextOutlineBuilder {
	pub offset_x: i32,
	pub offset_y: i32,
	pub commands: Vec<TextCommand>,
}

#[non_exhaustive]
#[derive(Debug)]
pub enum TextCommand {
	Move(Vec2),
	Line(Vec2),
	Arc { from: Vec2, to: Vec2, center: Vec2 },
}

#[cfg(feature = "type")]
impl OutlineBuilder for TextOutlineBuilder {
	fn move_to(&mut self, x: f32, y: f32) {
		self.commands.push(TextCommand::Move(Vec2 {
			x: x as F + self.offset_x as F,
			y: self.offset_y as F - y as F,
		}));
	}

	fn line_to(&mut self, x: f32, y: f32) {
		self.commands.push(TextCommand::Line(Vec2 {
			x: x as F + self.offset_x as F,
			y: self.offset_y as F - y as F,
		}));
	}

	fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
		self.commands.push(TextCommand::Line(Vec2 {
			x: x as F + self.offset_x as F,
			y: self.offset_y as F - y as F,
		}));
	}

	fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
		self.commands.push(TextCommand::Arc {
			from: Vec2 {
				x: x1 as F + self.offset_x as F,
				y: self.offset_y as F - y1 as F,
			},
			to: Vec2 {
				x: x2 as F + self.offset_x as F,
				y: self.offset_y as F - y2 as F,
			},
			center: Vec2 {
				x: x as F + self.offset_x as F,
				y: self.offset_y as F - y as F,
			},
		});
	}

	fn close(&mut self) {
		// No action required
	}
}

#[cfg(test)]
mod tests {
	#[cfg(feature = "type")]
	const FONT_DATA: &[u8] = include_bytes!("./Roboto-Regular.ttf");
	#[test]
	#[cfg(feature = "type")]
	fn text_outlines() {
		use rusttype::{point, Scale};

		use crate::TextOutlineBuilder;

		let font = rusttype::Font::try_from_bytes(FONT_DATA).unwrap();
		let glyphs = font.layout("Hello, world!", Scale { x: 10., y: 15. }, point(0.0, 0.0));
		let mut trace = TextOutlineBuilder::default();
		glyphs.for_each(|g| {
			trace.offset_x = g
				.pixel_bounding_box()
				.map(|v| v.min.x)
				.unwrap_or(trace.offset_x);
			trace.offset_y = g.pixel_bounding_box().map(|v| -v.min.y).unwrap_or(0);
			g.build_outline(&mut trace);
		});
		for i in trace.commands {
			println!(
				"{}",
				match i {
					crate::TextCommand::Move(p) | crate::TextCommand::Line(p) =>
						format!("{},{}", p.x, p.y),
					crate::TextCommand::Arc {
						from,
						to,
						center: _,
					} => format!("{},{}\n{},{}", from.x, from.y, to.x, to.y),
				}
			)
		}
	}
}

const PT_TO_MM: F = 0.35277777777777777777777777777778;

#[non_exhaustive]
#[derive(Debug)]
pub enum PenCommand {
	Move(Vec2),
	Up,
	Down,
}

impl From<TextOutlineBuilder> for Vec<PenCommand> {
	fn from(outline: TextOutlineBuilder) -> Self {
		let mut commands = vec![PenCommand::Up];
		let mut pen_down = false;
		for c in outline.commands {
			match c {
				TextCommand::Move(p) => {
					if pen_down {
						commands.push(PenCommand::Up);
						pen_down = false;
					}
					commands.push(PenCommand::Move(p * PT_TO_MM));
				}
				TextCommand::Line(p) => {
					if !pen_down {
						commands.push(PenCommand::Down);
						pen_down = true;
					}
					commands.push(PenCommand::Move(p * PT_TO_MM));
				}
				TextCommand::Arc { from, to, center } => {
					// I need to figure out how to actually compute this.
					// Currently it's just a straight line.
					if !pen_down {
						commands.push(PenCommand::Down);
						pen_down = true;
					}
					commands.push(PenCommand::Move(to * PT_TO_MM));
				}
			}
		}

		commands
	}
}
