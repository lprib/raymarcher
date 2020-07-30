use super::scene_object::SceneObject;
use crate::vec3::Vec3;
use cgmath::{Quaternion, InnerSpace, Zero, One};

type Quaternion64 = Quaternion<f64>;

const MAX_ITERS: i32 = 20;

pub struct Julia {
    pub c: Quaternion64,
    pub color: Vec3,
}

impl SceneObject for Julia {
    fn distance_to(&self, point: Vec3, t: f64) -> f64 {
        let mut z = Quaternion64::new(point.x, point.y, point.z, t);
        let mut dz = Quaternion64::new(1.0, 0.0, 0.0, 0.0);
        let mut count = 0;

        while count < MAX_ITERS {
            let z_new = z * z + self.c;
            dz = 2.0 * z * dz;
            z = z_new;

            if z.magnitude() > 4.0 {
                break;
            }
            count += 1;
        }

        let dist: f64 = z.magnitude() * z.magnitude().ln() / dz.magnitude();
        dist * 0.2
    }

    fn get_color(&self, t: f64) -> Vec3 {
        self.color
    }
}

pub struct Mandelbrot {
    pub w: f64,
    pub size: f64,
}

impl SceneObject for Mandelbrot {
    fn distance_to(&self, point: Vec3, _: f64) -> f64 {
        let c = Quaternion64::new(point.x, point.y, point.z, self.w);
        let mut z = Quaternion64::zero();
        let mut dz = Quaternion64::new(1.0, 0.0, 0.0, 0.0);
        let mut count = 0;

        while count < MAX_ITERS {
            let z_new = z * z + c;
            dz = 2.0 * z * dz + Quaternion64::one();
            z = z_new;

            if z.magnitude() > 4.0 {
                break;
            }
            count += 1;
        }

        let dist: f64 = z.magnitude() * z.magnitude().ln() / dz.magnitude();
        dist * 0.2
    }

    fn get_color(&self, _: f64) -> Vec3 {
        (0.5, 0.5, 1.0).into()
    }
}

pub struct Mandelbulb {
    pub color: Vec3
}


impl SceneObject for Mandelbulb {
    fn distance_to(&self, point: Vec3, t: f64) -> f64 {
        let power = 4.0;
        let mut z = point;
        let mut dr = 1.0;
        let mut r = 0.0;

        for i in 0..MAX_ITERS {
            r = z.magnitude();
            if r > 2.0 {
                break;
            }

            let mut theta = (z.z / r).acos();
            let mut phi = z.y.atan2(z.x);
            dr = r.powf(power - 1.0) * power * dr + 1.0;

            let zr = r.powf(power);
            theta = theta * power;
            phi = phi * power;

            z = zr * Vec3::from((theta.sin() * phi.cos(), phi.sin() * theta.sin(), theta.cos()));

            z = z + point;
        }

        0.5 * r * r.ln() / dr
    }

    fn get_color(&self, t: f64) -> Vec3 {
        self.color
    }
}