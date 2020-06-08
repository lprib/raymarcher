use common::vec3::Vec3;
use crate::scene::{SceneVec, Scene};
use crate::scene_object::SceneObject;

const MAX_STEPS: u32 = 200;
const HIT_THRESHOLD: f64 = 1E-3;

pub struct RayResult {
    pub len: f64,
    pub hit_point: Vec3,
}

pub fn cast_ray<T: SceneObject>(object: &T, point: Vec3, dir: Vec3) -> Option<RayResult> {
    let dir = dir.normalized();
    let mut current_point = point.clone();
    let mut iterations = 0u32;
    let mut t = 0.0;

    loop {
        // let (hit_index, radius) = scene.distance_to(current_point);
        let radius = object.distance_to(current_point);
        t += radius;
        iterations += 1;
        current_point = point + t * dir;
        if radius < HIT_THRESHOLD {
            return Some(RayResult {
                len: t,
                hit_point: current_point,
            });
        }

        if iterations > MAX_STEPS {
            return None;
        }
    }
}