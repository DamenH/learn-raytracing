use glam::Vec2;
use image::{ImageBuffer, Rgba, RgbaImage};

pub mod utils;

pub struct Renderer;

impl Renderer {
	pub fn render(width: u32, height: u32) -> RgbaImage {
		let img = ImageBuffer::from_fn(
			width as u32,
			height as u32,
			|x, y| {
				let frag_coord =
					Vec2::new(x as f32 / width as f32, y as f32 / height as f32);
				let color = Renderer::per_pixel(frag_coord);
				return Rgba([(color >> 16) as u8, (color >> 8) as u8, (color) as u8, 255]);
			},
		);
	
		return img;
	}

	fn per_pixel(coord: Vec2) -> u32 {
		let r = coord.x * 255.;
		let g = coord.y * 255.;
		let b = 0.;

		return (r as u32) << 16 | (g as u32) << 8 | (b as u32);
	}
}
