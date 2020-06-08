use crate::scene_object::SceneObject;
use common::vec3::Vec3;
use cgmath::{Quaternion, InnerSpace};

type Quaternion64 = Quaternion<f64>;

const JULIA_MAX_ITERS: i32 = 50;
const COMPLEX_PLANE_SIZE: f64 = 4.0;
const DISTANCE_ESTIMATE_MULTIPLIER: f64 = 0.5;

pub struct Julia {
    pub c: Quaternion64,
    /// Unconstrained extra dimension value for ray point
    pub w: f64,
    pub size: f64
}

impl SceneObject for Julia {
    fn distance_to(&self, point: Vec3) -> f64 {


        let mut z = Quaternion64::new(point.x / self.size, point.y / self.size, point.z / self.size, self.w / self.size);
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
        (0.5, 0.5, 1.0).into()
    }
}

fn world_to_complex(coord: f64) -> f64 {
    coord
}