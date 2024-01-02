use glam::{Vec3, Vec4};

use crate::{
    renderer::{
        scene::{Scene, Sphere},
        Renderer,
    },
    ui::window::draw_window,
};

pub mod renderer;
pub mod ui;

#[async_std::main]
async fn main() {
    let mut renderer = Renderer::new(Scene {
        camera: Default::default(),
        spheres: vec![Sphere {
            position: Vec3::new(0., 0., 0.),
            radius: 0.5,
            color: Vec4::new(1., 1., 1., 1.),
        }],
        lights: vec![Default::default()],
    });

    draw_window(500, 500, move |width: u32, height: u32, frame: u32| {
        renderer.render(width, height, frame)
    })
    .await;
}
