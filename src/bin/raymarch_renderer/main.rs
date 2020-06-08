use minifb::{Key, Window, WindowOptions};
use crate::raymarcher::RayMarcher;
use crate::fractals::{Julia, Mandelbulb};
use cgmath::Quaternion;
use crate::scene_object::Sphere;
use crate::sectioned::{ZSectioned};

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

    let mut window = Window::new(
        "Raymarcher",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    ).unwrap();

    let mut raymarcher = RayMarcher {
        object: Julia {
            c: Quaternion::new(-0.445,0.339,-0.0889,-0.562),
            w: 0.0,
            size: 1.0,
        },
        row: 0
    };
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

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut window_size = window.get_size();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        raymarcher.draw(buffer.as_mut_slice());
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        let new_size = window.get_size();
        if window_size != new_size {
            raymarcher.restart_render();
        }
        window_size = new_size;
    }
}

