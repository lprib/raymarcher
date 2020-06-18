use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use cgmath::Quaternion;

use crate::raymarcher::{RayMarcher, ImageRenderConfiguration};
use crate::fractals::{Julia, Mandelbulb};
use crate::scene_object::Sphere;
use crate::sectioned::{ZSectioned};
use std::path::Path;
use std::process::exit;

mod raymarcher;
mod scene;
mod scene_object;
mod ray;
mod fractals;
mod sectioned;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut raymarcher = RayMarcher {
        object: Julia {
            c: Quaternion::new(-1.0, 0.2, 0.0, 0.0),
            size: 1.0,
        },
        config: Default::default(),
    };

    // raymarcher.render_images(ImageRenderConfiguration {
    //     width: 128,
    //     height: 128,
    //     t_start: 0.0,
    //     t_stop: 1.0,
    //     t_step: 0.1,
    //     image_name: |i| format!("./images/test_image{}.png", i),
    // });

    let mut window = Window::new(
        "Raymarcher",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    ).unwrap();

    window.limit_update_rate(Some(Duration::from_micros(16600)));
    let mut row = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        raymarcher.draw(buffer.as_mut_slice(), row, (WIDTH, HEIGHT), 0.0);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        row += 1;
        if row >= HEIGHT {
            row = 0;
            println!("finished");
        }
    }
}