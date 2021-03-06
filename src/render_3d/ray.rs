use crate::vec3::Vec3;
use super::scene_object::SceneObject;

const MAX_STEPS: u32 = 200;
const HIT_THRESHOLD: f64 = 1E-4;

#[derive(Debug)]
pub struct RayResult {
    pub len: f64,
    pub hit_point: Vec3,
}

pub fn cast_ray<O: SceneObject>(object: &O, point: Vec3, dir: Vec3, backplanes: Vec3, t: f64) -> Option<RayResult> {
    let dir = dir.normalized();
    let mut current_point = point.clone();
    let mut iterations = 0u32;
    let mut ray_len = 0.0;

    loop {
        let radius = object.distance_to(current_point, t);
        ray_len += radius;
        iterations += 1;
        current_point = point + ray_len * dir;
        if radius < HIT_THRESHOLD {
            return Some(RayResult {
                len: ray_len,
                hit_point: current_point,
            });
        }

        // if iterations > MAX_STEPS {
        //     return None;
        // }

        if current_point.x.abs() > backplanes.x ||
            current_point.y.abs() > backplanes.y ||
            current_point.z.abs() > backplanes.z {
            return None;
        }
    }
}