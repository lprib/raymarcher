use minifb::{Key, Window, WindowOptions};
use crate::raymarcher::RayMarcher;
use crate::julia::Julia;
use cgmath::Quaternion;
use crate::scene_object::Sphere;
use crate::sectioned::{ZSectioned};

mod raymarcher;
mod scene;
mod scene_object;
mod ray;
mod julia;
mod sectioned;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

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

    // let raymarcher = RayMarcher {
    //     object: Julia {
    //         c: Quaternion::new(-1.0, 0.2, 0.0, 0.0),
    //         w: 0.0,
    //         size: 2.0,
    //     }
    // };

    let raymarcher = RayMarcher {
        object: ZSectioned {
            z: 0.0,
            object: Julia {
                c: Quaternion::new(-1.0, 0.2, 0.0, 0.0),
                w: 0.0,
                size: 2.0,
            },
        }
    };

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        raymarcher.draw(buffer.as_mut_slice());
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

