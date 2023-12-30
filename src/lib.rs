use glam::Vec2;
use image::{ImageBuffer, Rgb, RgbImage};

mod utils;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 600;

struct Circle {
    r: f32,
}

struct Ray {
    origin: Vec2,
    direction: Vec2,
}

pub fn render() -> RgbImage {
    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_WIDTH);

    let circle = Circle { r: 150. };
    let offset_x = (IMAGE_WIDTH / 2) as i32;
    let offset_y = (IMAGE_HEIGHT / 2) as i32 + (circle.r / 2.) as i32;

    let ray = Ray {
        origin: Vec2::new(-offset_x as f32, -500.),
        direction: Vec2::new(0., 1.),
    };

    for ray_offset in 0..IMAGE_WIDTH {
        let ray_x = ray.origin.x + ray_offset as f32;

        let a = ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y;
        let b = 2. * (ray.direction.x * ray_x + ray.direction.y * ray.origin.y);
        let c = ray_x * ray_x + ray.origin.y * ray.origin.y - circle.r * circle.r;

        let descriminant = b * b - 4. * a * c;
        if descriminant < 0. {
            continue; // No intersection
        }

        let t = (-b + descriminant.sqrt()) / (2. * a);
        let x = (ray_x + ray.direction.x * t) as i32 + offset_x;
        let y = (ray.origin.y + ray.direction.y * t) as i32 + offset_y;
        img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));

		let t = (-b - descriminant.sqrt()) / (2. * a);
        let x = (ray_x + ray.direction.x * t) as i32 + offset_x;
        let y = (ray.origin.y + ray.direction.y * t) as i32 + offset_y;
        img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
    }

    return img;
}
