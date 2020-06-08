use crate::scene_object::SceneObject;
use common::vec3::Vec3;
use cgmath::{Quaternion, InnerSpace, Zero, One};

type Quaternion64 = Quaternion<f64>;

const JULIA_MAX_ITERS: i32 = 100;
const COMPLEX_PLANE_SIZE: f64 = 2.0;
const DISTANCE_ESTIMATE_MULTIPLIER: f64 = 0.5;

const COLOR: Vec3 = Vec3 { x: 1.0, y: 0.5, z: 0.5 };

pub struct Julia {
    pub c: Quaternion64,
    /// Unconstrained extra dimension value for ray point
    pub w: f64,
    pub size: f64,
}

impl SceneObject for Julia {
    fn distance_to(&self, point: Vec3) -> f64 {
        let mut z = (1.0 / self.size) * Quaternion64::new(point.x, point.y, point.z, self.w);
        // println!("{:?}", z);

        let mut dz = Quaternion64::new(1.0, 0.0, 0.0, 0.0);
        let mut count = 0;

        loop {
            let z_new = z * z + self.c;
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
        let dist = if dist < 0.0 { 0.0 } else { dist };
        dist * 0.2
        // (point).magnitude() - 1.0
    }

    fn get_color(&self) -> Vec3 {
        COLOR
    }
}

pub struct Mandelbulb {
    pub w: f64,
    pub size: f64,
}

impl SceneObject for Mandelbulb {
    fn distance_to(&self, point: Vec3) -> f64 {
        let c = (1.0 / self.size) * Quaternion64::new(point.x, point.y, point.z, self.w);
        let mut z = Quaternion64::zero();
        let mut dz = Quaternion64::new(1.0, 0.0, 0.0, 0.0);
        let mut count = 0;

        loop {
            let z_new = z * z + c;
            dz = 2.0 * z * dz + Quaternion64::one();
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
        let dist = if dist < 0.0 { 0.0 } else { dist };
        dist * 0.2
    }

    fn get_color(&self) -> Vec3 {
        (0.5, 0.5, 1.0).into()
    }
}