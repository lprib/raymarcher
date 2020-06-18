use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use cgmath::Quaternion;
use std::path::Path;
use std::process::exit;

use raymarcher::{RayMarcher, RayMarcherConfig, ImageRenderConfiguration};
use fractals::{Julia, Mandelbrot};
use scene_object::Sphere;
use sectioned::{ZSectioned};
use crate::render_3d::fractals::Mandelbulb;
use crate::render_3d::scene_object::SceneObject;

pub mod raymarcher;
mod scene;
mod scene_object;
mod ray;
pub mod fractals;
mod sectioned;

pub fn main<O: SceneObject>(
    width: usize,
    height: usize,
    config: RayMarcherConfig, object: O) {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut raymarcher = RayMarcher {
        object,
        config,
    };

    let mut window = Window::new(
        "Raymarcher",
        width,
        height,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    ).unwrap();

    window.limit_update_rate(Some(Duration::from_micros(16600)));
    let mut row = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        raymarcher.draw(buffer.as_mut_slice(), row, (width, height), 0.0);
        window.update_with_buffer(&buffer, width, height).unwrap();

        row += 1;
        if row >= height {
            row = 0;
            println!("finished");
        }
    }
}