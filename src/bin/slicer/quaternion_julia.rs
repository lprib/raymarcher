use common::vec3::Vec3;
use cgmath::{Quaternion, InnerSpace, Zero};

type Quaternion64 = Quaternion<f64>;

static C: Quaternion64 = Quaternion::new(-0.291, -0.399, 0.339, 0.437);

const JULIA_MAX_ITERS: i32 = 100;
const COMPLEX_PLANE_SIZE: f64 = 4.0;

static MAX_RAYMARCH_ITERS: i32 = 500;
static RAYMARCH_HIT_THRESHOLD: f64 = 0.01;

fn get_z(x: f64, y: f64, px: f64, py: f64) -> Quaternion64 {
    Quaternion::new(x, y, px, py)
}

fn get_val(z: Quaternion64) -> f64 {
    let mut count = 0;
    let mut z = z;

    loop {
        z = z * z + C;
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

fn get_val_dist_estimate(z: Quaternion64) -> f64 {
    let mut count = 0;
    let mut z = z;
    let mut dz = Quaternion64::new(1.0, 0.0, 0.0, 0.0);

    loop {
        let z_new = z*z + C;
        dz = 2.0 * z * dz;
        z = z_new;

        if z.magnitude() > 4.0 {
            break;
        }
        count += 1;
        if count > JULIA_MAX_ITERS {
            break;
        }
    }

    let dist: f64 = z.magnitude() * z.magnitude().ln() / dz.magnitude();
    let dist = if dist < 0.0 {0.0} else {dist};
    // let dist = dist.abs();
    // dbg!(dist);
    let log_dist = dist.ln();
    log_dist;
    dist
}

fn get_val_mandelbrot(pos: Quaternion64) -> f64 {
    let mut count = 0;
    let mut z = Quaternion64::zero();

    loop {
        z = z * z + pos;
        if z.magnitude() > 3.0 {
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

    trace_ray(frame, mouse_x, mouse_y);
}

fn trace_ray(frame: &mut[u32], mouse_x: f64, mouse_y: f64) {
    let mouse_plane_x = mouse_x / super::WIDTH as f64;
    let mouse_plane_y = mouse_y / super::HEIGHT as f64;

    let start_point = Vec3::from((0, 0, 0));
    let mut current_point = start_point.clone();
    let dir = (Vec3::from((mouse_x, mouse_y, 0.0)) - start_point).normalized();
    let mut t = 0.0;
    let mut iters = 0;

    loop {
        let (zx, zy) = screen_to_complex(current_point.x, current_point.y);
        let radius = get_val_dist_estimate(get_z(zx, zy, mouse_plane_x, mouse_plane_y)) * 30.0;
        t += radius;
        iters += 1;
        current_point = start_point + t*dir;
        draw_point(frame, current_point.x, current_point.y, (1, 0, 0).into());

        if radius < RAYMARCH_HIT_THRESHOLD {
            break;
        }
        if iters > MAX_RAYMARCH_ITERS {
            break;
        }
    }

    // dbg!(current_point);
}

fn screen_to_complex(x: f64, y: f64) -> (f64, f64) {
    let x = (x / super::WIDTH as f64) * COMPLEX_PLANE_SIZE - COMPLEX_PLANE_SIZE / 2.0;
    let y = (y / super::HEIGHT as f64) * COMPLEX_PLANE_SIZE - COMPLEX_PLANE_SIZE / 2.0;
    (x, y)
}

fn draw_point(frame: &mut [u32], x: f64, y: f64, color: Vec3) {
    if x >= 0.0 && y >= 0.0 && x < super::WIDTH as f64 && y < super::HEIGHT as f64 {
        frame[y as usize * super::WIDTH + x as usize] = color.into();
    }
}