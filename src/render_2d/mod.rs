use minifb::{Key, Window, WindowOptions, MouseMode};
use cgmath::Quaternion;

mod julia_distance;
mod quaternion_julia;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

pub fn main(c: Quaternion<f64>) {
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

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let window_size = window.get_size();
        let mouse_absolute = window.get_mouse_pos(MouseMode::Clamp).unwrap();
        let mouse_scaled =  (
            mouse_absolute.0 / window_size.0 as f32 * WIDTH as f32,
            mouse_absolute.1 / window_size.1 as f32 * HEIGHT as f32
        );

        // julia_distance::draw_ray2d(buffer.as_mut_slice(), mouse_scaled.0 as f64, mouse_scaled.1 as f64);
        quaternion_julia::draw_quaternion_julia(buffer.as_mut_slice(), mouse_scaled.0 as f64, mouse_scaled.1 as f64, c);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

