use common::vec3::Vec3;
use cgmath::{Quaternion, InnerSpace};

type Quaternion64 = Quaternion<f64>;

static C: Quaternion64 = Quaternion::new(-0.291,-0.399,0.339,0.437);

const JULIA_MAX_ITERS: i32 = 100;
const COMPLEX_PLANE_SIZE: f64 = 2.0;

fn get_z(x: f64, y: f64, px: f64, py: f64) -> Quaternion64 {
    Quaternion::new(x, y, px, py)
}

fn get_val(z: Quaternion64) -> f64 {
    let mut count = 0;
    let mut z = z;

    loop {
        z = z * z  + C;
        if z.magnitude() > 4.0 {
            break;
        }
        count += 1;
        if count > JULIA_MAX_ITERS {
            break;
        }
    }
    count as f64 / JULIA_MAX_ITERS as f64
}

pub fn draw_quaternion_julia(frame: &mut [u32], mouse_x: f64, mouse_y: f64) {
    for (i, pix) in frame.iter_mut().enumerate() {
        let x = i % super::WIDTH;
        let y = i / super::WIDTH;

        let x = (x as f64 / super::WIDTH as f64) * COMPLEX_PLANE_SIZE - COMPLEX_PLANE_SIZE / 2.0;
        let y = (y as f64 / super::HEIGHT as f64) * COMPLEX_PLANE_SIZE - COMPLEX_PLANE_SIZE / 2.0;

        let mouse_plane_x = mouse_x / super::WIDTH as f64;
        let mouse_plane_y = mouse_y / super::HEIGHT as f64;
        let val = get_val(get_z(x, y, mouse_plane_x, mouse_plane_y));
        *pix = Vec3::from(val).into();
    }
}