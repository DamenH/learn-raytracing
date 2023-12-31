use crate::{renderer::scene::Scene, ui::window::draw_window};

pub mod renderer;
pub mod ui;

#[async_std::main]
async fn main() {
    draw_window(Scene {
        camera: Default::default(),
        spheres: vec![Default::default()],
        lights: vec![Default::default()],
    })
    .await;
}
