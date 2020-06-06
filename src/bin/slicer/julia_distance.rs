use num_complex::Complex64;
use common::vec3::Vec3;

static C: Complex64 = Complex64::new(-0.1, 0.651);
// static C: Complex64 = Complex64::new(0.0, 0.0);
static MAX_JULIA_ITERS: i32 = 500;
static MAX_RAYMARCH_ITERS: i32 = 500;
static RAYMARCH_HIT_THRESHOLD: f64 = 0.01;

static mut POINTS: Vec<Vec3> = Vec::new();
static RED: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
static BLUE: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

pub fn draw(frame: &mut [u32]) {
    // let mut C = Complex64::new(-0.1, 0.651);

    for (i, pix) in frame.iter_mut().enumerate() {
        let x = i % super::WIDTH;
        let y = i / super::WIDTH;

        let x = (x as f64 / super::WIDTH as f64) * 4.0 - 2.0;
        let y = (y as f64 / super::HEIGHT as f64) * 4.0 - 2.0;


        let mut z = Complex64::new(x, y);
        let mut dz = Complex64::new(1.0, 0.0);
        let mut count = 0;

        loop {
            let z_new = z * z + C;
            // let z_new = 1.0 - z.powi(2) + z.powi(5)/(2.0 + 4.0*z) + C;
            dz = 2.0 * z * dz;
            z = z_new;

            if z.norm() > 1.5 {
                break;
            }
            count += 1;
            if count > MAX_JULIA_ITERS {
                break;
            }
        }

        let dist: f64 = z.norm() * z.norm().ln() / dz.norm();
        let dist = if dist < 0.0 { 0.0 } else { dist };
        let dist = dist / 4.0;
        let logdist = -dist.ln();
        *pix = Vec3::from_f64(logdist / 20.0).into();
        // *pix = Vec3::from_f64(count as f64 / max_iters as f64).into();
        // *pix = if logdist > 13.0 {Vec3::from_f64(1.0).into()} else {Vec3::from_f64(0.0).into()};
    }
}

pub fn dist_estimate(zx: f64, zy: f64) -> f64 {
    let mut z = Complex64::new(zx, zy);
    let mut dz = Complex64::new(1.0, 0.0);
    let mut count = 0;

    loop {
        let z_new = z * z + C;
        dz = 2.0 * z * dz;
        z = z_new;

        if z.norm() > 1.5 {
            break;
        }
        count += 1;
        if count > MAX_JULIA_ITERS {
            break;
        }
    }

    let dist: f64 = z.norm() * z.norm().ln() / dz.norm();
    let dist = if dist < 0.0 { 0.0 } else { dist };
    dist
}

pub fn draw_ray2d(frame: &mut [u32], mouse_x: f64, mouse_y: f64) {
    // draw set bg:
    for (i, pix) in frame.iter_mut().enumerate() {
        let x = i % super::WIDTH;
        let y = i / super::WIDTH;

        let x = (x as f64 / super::WIDTH as f64) * 4.0 - 2.0;
        let y = (y as f64 / super::HEIGHT as f64) * 4.0 - 2.0;

        let dist = dist_estimate(x, y);
        let logdist = -dist.ln();
        *pix = Vec3::from_f64(logdist / 15.0).into();
        *pix = Vec3::from_f64(dist).into();
    }

    //march rays
    let start_point = Vec3::from((0, super::WIDTH as i32 / 2, 0));
    let dir = (Vec3::from((mouse_x, mouse_y, 0.0)) - start_point).normalized();

    let mut current_point = start_point.clone();
    let mut t = 0.0;
    let mut iters = 0;

    let was_hit = loop {
        let (zx, zy) = screen_to_complex(current_point.x, current_point.y);
        let radius = dist_estimate(zx, zy) * 40.0;
        t += radius;
        iters += 1;
        current_point = start_point + t * dir;
        draw_point(frame, current_point.x, current_point.y, RED);
        if radius < RAYMARCH_HIT_THRESHOLD {
            break true;
        }
        if iters > MAX_RAYMARCH_ITERS {
            break false;
        }
    };

    unsafe {
        if was_hit {
            POINTS.push(current_point);
        }

        for p in POINTS.iter() {
            draw_point(frame, p.x, p.y, BLUE);
        }
    }
}

fn draw_point(frame: &mut [u32], x: f64, y: f64, color: Vec3) {
    if x >= 0.0 && y >= 0.0 && x < super::WIDTH as f64 && y < super::HEIGHT as f64 {
        frame[y as usize * super::WIDTH + x as usize] = color.into();
    }
}

fn screen_to_complex(x: f64, y: f64) -> (f64, f64) {
    (x / super::WIDTH as f64 * 4.0 - 2.0, y / super::HEIGHT as f64 * 4.0 - 2.0)
}