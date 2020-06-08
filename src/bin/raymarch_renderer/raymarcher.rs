use common::vec3::Vec3;
use crate::scene::{SceneVec, Scene};
use crate::ray::cast_ray;
use crate::scene_object::{Sphere, SceneObject};
use crate::fractals::Julia;
use cgmath::Quaternion;
use rayon::prelude::*;

const SHININESS: f64 = 50.0;

pub struct RayMarcher<T: SceneObject> {
    pub object: T,
    pub row: usize,
}

impl<T: SceneObject> RayMarcher<T> {
    const CAMERA_POS: Vec3 = Vec3 { x: 2.0, y: 4.0, z: 4.0 };
    const LOOK_AT: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    const LIGHT_POS: Vec3 = Vec3 { x: 2.0, y: 4.0, z: 4.0 };
    const BG_COLOR: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    const ZOOM: f64 = 4.0;
    const AA_LEVEL: i32 = 4;
    const BACKPLANES: Vec3 = Vec3 { x: 3.0, y: 3.0, z: 3.0 };

    pub fn draw(&mut self, frame: &mut [u32]) {
        frame
            .par_iter_mut()
            .enumerate()
            .skip(self.row * super::WIDTH)
            .take(super::WIDTH)
            .for_each(|(i, pix)| {
                *pix = self.send_pixel_ray(i).into()
            });

        self.row += 1;
        if self.row == super::HEIGHT {
            self.row = 0;
        }
    }

    pub fn restart_render(&mut self) {
        self.row = 0;
    }


    fn send_pixel_ray(&self, buffer_idx: usize) -> Vec3 {
        let x = buffer_idx % super::WIDTH;
        let y = buffer_idx / super::HEIGHT;

        let subpixel_size = 1.0 / Self::AA_LEVEL as f64;
        let mut pixel_sum = Vec3::default();
        for subpixel_x in 0..Self::AA_LEVEL {
            for subpixel_y in 0..Self::AA_LEVEL {
                let ray_dir = Self::camera_ray_dir(
                    x as f64 + subpixel_x as f64 * subpixel_size,
                    y as f64 + subpixel_y as f64 * subpixel_size,
                    Self::CAMERA_POS,
                    Self::LOOK_AT,
                    Self::ZOOM,
                );
                pixel_sum = pixel_sum + self.trace(Self::CAMERA_POS, ray_dir);
            }
        }
        (1.0 / (Self::AA_LEVEL * Self::AA_LEVEL) as f64) * pixel_sum

        // let ray_dir = Self::camera_ray_dir(x as f64, y as f64, Self::CAMERA_POS, Self::LOOK_AT, Self::ZOOM);
        // self.trace(Self::CAMERA_POS, ray_dir)
    }

    fn trace(&self, point: Vec3, dir: Vec3) -> Vec3 {
        let res = cast_ray(&self.object, point, dir, Self::BACKPLANES);
        match res {
            Some(res) => {
                let light_vec = (Self::LIGHT_POS - res.hit_point).normalized();
                let norm = self.object.normal(res.hit_point);
                let s_dot_n = norm.dot(light_vec);

                //specularity
                let reflect_vec = (-light_vec).reflect(norm);
                let view_vec = Self::CAMERA_POS - res.hit_point;
                let r_dot_v = reflect_vec.dot(view_vec.normalized());
                let specular_term = r_dot_v.powf(SHININESS);
                let specular_term = if r_dot_v > 0.0 { specular_term } else { 0.0 };

                s_dot_n * self.object.get_color() + specular_term * Vec3::from((1, 1, 1))
            }
            None => Self::BG_COLOR
        }
    }

    fn camera_ray_dir(x: f64, y: f64, cam_pos: Vec3, look_at: Vec3, zoom: f64) -> Vec3 {
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