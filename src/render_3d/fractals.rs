use super::scene_object::SceneObject;
use crate::vec3::Vec3;
use cgmath::{Quaternion, InnerSpace, Zero, One};

type Quaternion64 = Quaternion<f64>;

const JULIA_MAX_ITERS: i32 = 100;
const COMPLEX_PLANE_SIZE: f64 = 2.0;
const DISTANCE_ESTIMATE_MULTIPLIER: f64 = 0.5;

const COLOR: Vec3 = Vec3 { x:0.5, y: 0.5, z: 0.5 };

pub struct Julia {
    pub c: Quaternion64,
    //todo properly implement transformations
    pub size: f64,
}

impl SceneObject for Julia {
    fn distance_to(&self, point: Vec3, t: f64) -> f64 {
        let mut z = Quaternion64::new(point.x, point.y, point.z, t);
        let mut dz = Quaternion64::new(1.0, 0.0, 0.0, 0.0);
        let mut count = 0;

        while count < JULIA_MAX_ITERS {
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
        COLOR
    }
}

pub struct Mandelbulb {
    pub w: f64,
    pub size: f64,
}

impl SceneObject for Mandelbulb {
    fn distance_to(&self, point: Vec3, _: f64) -> f64 {
        let c = Quaternion64::new(point.x, point.y, point.z, self.w);
        let mut z = Quaternion64::zero();
        let mut dz = Quaternion64::new(1.0, 0.0, 0.0, 0.0);
        let mut count = 0;

        while count < JULIA_MAX_ITERS {
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