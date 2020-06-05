use minifb::{Key, Window, WindowOptions};
use crate::raymarcher::RayMarcher;

mod raymarcher;
mod vec3;
mod scene;
mod scene_object;
mod ray;
mod julia_distance;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Raymarcher",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    ).unwrap();

    let raymarcher = RayMarcher::new();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        raymarcher.draw(buffer.as_mut_slice());
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}