use common::vec3::Vec3;
use crate::scene::{SceneVec, Scene};
use crate::ray::cast_ray;
use crate::scene_object::{Sphere, SceneObject};
use crate::julia::Julia;
use cgmath::Quaternion;
use rayon::prelude::*;


pub struct RayMarcher<T: SceneObject> {
    pub(crate) object: T
}

impl<T: SceneObject> RayMarcher<T> {
    const CAMERA_POS: Vec3 = Vec3 { x: 4.0, y: 4.0, z: 4.0 };
    const LOOK_AT: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    const LIGHT_POS: Vec3 = Vec3 { x: 4.0, y: 4.0, z: 2.0 };
    const BG_COLOR: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn draw(&self, frame: &mut [u32]) {
        // dbg!(self.scene.distance_to((4, 4, 4).into()));
        // dbg!(self.scene.distance_to((-4, -4, -4).into()));
        // self.trace((4, 4, 4).into(), Vec3::from((-1, -1, -1)).normalized());
        frame.par_iter_mut().enumerate().for_each(|(i, pix)| {
            *pix = self.send_pixel_ray(i).into()
        });
        println!("frame");
    }

    fn send_pixel_ray(&self, buffer_idx: usize) -> Vec3 {
        let x = buffer_idx % super::WIDTH;
        let y = buffer_idx / super::HEIGHT;

        let ray_dir = Self::camera_ray_dir(x, y, Self::CAMERA_POS, Self::LOOK_AT, 1.0);
        self.trace(Self::CAMERA_POS, ray_dir)
    }

    fn trace(&self, point: Vec3, dir: Vec3) -> Vec3 {
        let res = cast_ray(&self.object, point, dir);
        match res {
            Some(res) => {
                let norm = self.object.normal(res.hit_point);
                let s_dot_n = norm.dot((Self::LIGHT_POS - res.hit_point).normalized());
                s_dot_n * self.object.get_color()
            }
            None => Self::BG_COLOR
        }
    }

    fn camera_ray_dir(x: usize, y: usize, cam_pos: Vec3, look_at: Vec3, zoom: f64) -> Vec3 {
        let u = -(x as f64 / super::WIDTH as f64 * 2.0 - 1.0);
        let v = y as f64 / super::HEIGHT as f64 * 2.0 - 1.0;

        let look_dir = (look_at - cam_pos).normalized();
        let right_vec = Vec3::from((0, -1, 0)).cross(look_dir);
        let down_vec = look_dir.cross(right_vec);

        let zoomed_cam_pos = cam_pos + zoom * look_dir;
        let pix_pos = zoomed_cam_pos + u * right_vec + v * down_vec;
        let dir = pix_pos - cam_pos;
        dir.normalized()
    }
}