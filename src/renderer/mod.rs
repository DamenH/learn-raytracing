use self::scene::Scene;
use glam::{Vec2, Vec3, Vec4};
use image::{ImageBuffer, RgbaImage};
use rayon::prelude::*;

pub mod scene;

#[derive(Clone)]
pub struct Renderer {
    pub scene: Scene,
    pub frame: u32,
}

impl Renderer {
    pub fn new(scene: Scene) -> Self {
        Self { scene, frame: 0 }
    }

    pub fn render(&mut self, width: u32, height: u32, frame: u32) -> RgbaImage {
        self.frame = frame;
        println!("Frame: {}", frame);

        let mut buffer = vec![0u8; (width * height * 4) as usize];

        buffer
            .par_chunks_mut(4)
            .enumerate()
            .for_each(|(index, slice)| {
                let frag_x = (index as u32 % width) as f32 / width as f32;
                let frag_y = (index as u32 / width) as f32 / height as f32;
                let frag_coord = Vec2::new(frag_x, frag_y) * 2. - 1.;
                let color = self.per_pixel(frag_coord);
                slice[0] = (color.x * 255.99) as u8;
                slice[1] = (color.y * 255.99) as u8;
                slice[2] = (color.z * 255.99) as u8;
                slice[3] = (color.w * 255.99) as u8;
            });

        ImageBuffer::from_vec(width, height, buffer).unwrap()
    }

    // TODO: Aspect Ratio, Camera View Matrix Projection
    // TODO: Be able to move the sphere around
    // TODO: Be able to move the camera around
    // TODO: Add Light Primative
    // TODO: Sphere Collision Cordinates
    // TODO: Sphere Collision Normal
    // TODO: Sphere Collision Reflection

    fn per_pixel(&self, coord: Vec2) -> Vec4 {
        let ray_origin = self.scene.camera.position;
        let ray_direction = (self.scene.camera.rotation
            + Vec3::new(coord.x, coord.y, -1.)
            + Vec3::new(0., (self.frame as f32) * 0.01, 0.))
        .normalize();

        let sphere = &self.scene.spheres[0];

        // (bx^2 + by^2 + bz^2)t^2 + (2(axbx + ayby + azbz))t + (ax^2 + ay^2 + az^2 - r^2) = 0
        // ax^2 + bx + c = 0 ... quadratic equation
        // a = ray origin
        // b = ray direction
        // r = sphere radius
        // t = distance along ray

        let radius = sphere.radius;

        let a = ray_direction.dot(ray_direction);
        let b = 2. * ray_origin.dot(ray_direction);
        let c = ray_origin.dot(ray_origin) - radius * radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return Vec4::new(0., 0., 0., 0.);
        }

        // let t0 = -b - discriminant.sqrt() / (2. * a);
        // let t1 = -b + discriminant.sqrt() / (2. * a);

        // let hit_position_0 = ray_origin + ray_direction * t0;
        // let hit_position_1 = ray_origin + ray_direction * t1;

        // let hit_normal_0 = (hit_position_0 - Vec3::new(0., 0., 0.)).normalize();

        Vec4::new(1., 0., 0., 1.)
    }
}
