use glam::{Vec2, Vec3, Vec4};
use image::{ImageBuffer, Rgba, RgbaImage};

use self::scene::Scene;

pub mod scene;

pub struct Renderer;

impl Renderer {
    pub fn render(width: u32, height: u32, scene: Scene) -> RgbaImage {
        let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            let frag_coord = Vec2::new(x as f32 / width as f32, y as f32 / height as f32) * 2. - 1.;
            let color = Renderer::per_pixel(frag_coord);
            return Rgba([
                (color.x * 255.99) as u8,
                (color.y * 255.99) as u8,
                (color.z * 255.99) as u8,
                (color.w * 255.99) as u8,
            ]);
        });

        return img;
    }

    // TODO: Aspect Ratio, Camera View Matrix Projection
    // TODO: Be able to move the sphere around
    // TODO: Be able to move the camera around
    // TODO: Add Light Primative
    // TODO: Sphere Collision Cordinates
    // TODO: Sphere Collision Normal
    // TODO: Sphere Collision Reflection

    fn per_pixel(coord: Vec2) -> Vec4 {
        let ray_origin = Vec3::new(0., 0., 2.);
        let ray_direction = Vec3::new(coord.x, coord.y, -1.).normalize();
        let radius = 0.5_f32;

        // (bx^2 + by^2 + bz^2)t^2 + (2(axbx + ayby + azbz))t + (ax^2 + ay^2 + az^2 - r^2) = 0
        // ax^2 + bx + c = 0 ... quadratic equation
        // a = ray origin
        // b = ray direction
        // r = sphere radius
        // t = distance along ray

        let a = ray_direction.dot(ray_direction);
        let b = 2. * ray_origin.dot(ray_direction);
        let c = ray_origin.dot(ray_origin) - radius * radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return Vec4::new(0., 0., 0., 0.);
        }

		let t0 = -b - discriminant.sqrt() / (2. * a);
		let t1 = -b + discriminant.sqrt() / (2. * a);

		let hit_position_0 = ray_origin + ray_direction * t0;
		let hit_position_1 = ray_origin + ray_direction * t1;

		let hit_normal_0 = (hit_position_0 - Vec3::new(0., 0., 0.)).normalize();



        return Vec4::new(1., 0., 0., 1.);
    }
}
