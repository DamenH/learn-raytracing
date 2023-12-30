use image::{RgbImage, ImageBuffer};

mod utils;

pub fn render() -> RgbImage {
	ImageBuffer::from_fn(800, 600, |x, y| {
		image::Rgb([x as u8, y as u8, 0])
	})
}
