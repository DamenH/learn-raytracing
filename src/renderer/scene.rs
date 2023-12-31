use glam::{Vec3, Vec4};

pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			position: Vec3::new(0., 0., 2.),
			rotation: Vec3::new(0., 0., 0.),
			fov: 90.,
			aspect_ratio: 1.,
			near: 0.1,
			far: 100.,
		}
	}
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: Vec4,
}

impl Default for Sphere {
	fn default() -> Self {
		Self {
			position: Vec3::new(0., 0., 0.),
			radius: 0.5,
			color: Vec4::new(1., 1., 1., 1.),
		}
	}
}

pub struct Light {
    pub position: Vec3,
    pub color: Vec4,
}

impl Default for Light {
	fn default() -> Self {
		Self {
			position: Vec3::new(0., 0., 0.),
			color: Vec4::new(1., 1., 1., 1.),
		}
	}
}