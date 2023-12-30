use std::time::{SystemTime, UNIX_EPOCH};
use learn_raytracing::render;

fn main() {
    let img = render();

	let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

	img.save(format!("renders/output-{}.png", since_the_epoch.as_secs() * 1000)).unwrap();
}
