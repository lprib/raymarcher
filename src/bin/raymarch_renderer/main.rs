use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use cgmath::Quaternion;

use crate::raymarcher::RayMarcher;
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

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut raymarcher = RayMarcher {
        object: Julia {
            c: Quaternion::new(-1.0,0.2,0.0,0.0),
            size: 1.0,
        },
        row: 0,
    };
    render_images();

    // raymarcher.render_to_image(Path::new("./image.png"), 0.95);

    //
    // let raymarcher = RayMarcher {
    //     object: Mandelbulb {
    //         w: 1.0,
    //         size: 2.0,
    //     }
    // };

    // let raymarcher = RayMarcher {
    //     object: Sphere {
    //         center: (0, 0, 0).into(),
    //         radius: 1.0,
    //         color: (0, 0, 1).into(),
    //     }
    // };

    // let mut raymarcher = RayMarcher {
    //     object: ZSectioned {
    //         z: 1.0,
    //         object: Mandelbulb {
    //             w: 0.0,
    //             size: 2.0,
    //         }
    //     },
    //     row: 0
    // };

    exit(0);

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        raymarcher.draw(buffer.as_mut_slice(), 0.66);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn render_images() {
    let mut raymarcher = RayMarcher {
        object: Julia {
            c: Quaternion::new(-1.0,0.2,0.0,0.0),
            size: 1.0,
        },
        row: 0,
    };

    let mut t = 0.0;
    let mut i = 0;

    while t < 0.66 {
        let image_name = format!("./images/image{}.png", i);
        let path = Path::new(&image_name);
        raymarcher.render_to_image(path, t);

        i += 1;
        t += 0.005;
        println!("rendered image {}", image_name);
    }
}