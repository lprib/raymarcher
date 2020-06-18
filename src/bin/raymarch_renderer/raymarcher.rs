use common::vec3::Vec3;
use crate::scene::{SceneVec, Scene};
use crate::ray::cast_ray;
use crate::scene_object::{Sphere, SceneObject};
use crate::fractals::Julia;
use cgmath::Quaternion;
use rayon::prelude::*;
use std::path::Path;
use image::{DynamicImage, GenericImage, ImageFormat, Rgba};

pub struct RayMarcher<O: SceneObject> {
    pub object: O,
    pub config: RayMarcherConfig,
}

impl<O: SceneObject> RayMarcher<O> {
    pub fn draw(&mut self, frame: &mut [u32], row: usize, (width, height): (usize, usize), t: f64) {
        frame
            .par_iter_mut()
            .enumerate()
            .skip(row * width)
            .take(width)
            .for_each(|(i, pix)| {
                *pix = self.send_pixel_ray(i, (width, height), t).into()
            });
    }

    fn send_pixel_ray(&self, buffer_idx: usize, (width, height): (usize, usize), t: f64) -> Vec3 {
        let x = buffer_idx % width;
        let y = buffer_idx / width;

        let aa_level = self.config.anti_aliasing_level;

        let subpixel_size = 1.0 / aa_level as f64;
        let mut pixel_sum = Vec3::default();
        for subpixel_x in 0..aa_level {
            for subpixel_y in 0..aa_level {
                let ray_dir = Self::camera_ray_dir(
                    x as f64 + subpixel_x as f64 * subpixel_size,
                    y as f64 + subpixel_y as f64 * subpixel_size,
                    self.config.camera_pos,
                    self.config.look_at,
                    self.config.camera_zoom,
                    (width, height),
                );
                pixel_sum = pixel_sum + self.trace(self.config.camera_pos, ray_dir, t);
            }
        }
        (1.0 / (aa_level * aa_level) as f64) * pixel_sum
    }

    fn trace(&self, point: Vec3, dir: Vec3, t: f64) -> Vec3 {
        let res = cast_ray(&self.object, point, dir, self.config.backplane_positions, t);
        match res {
            Some(res) => {
                // if there is a ray hit, do Phong lighting calculations
                let light_vec = (self.config.light_pos - res.hit_point).normalized();
                let norm = self.object.normal(res.hit_point, t);
                let s_dot_n = norm.dot(light_vec);

                //specularity
                let reflect_vec = (-light_vec).reflect(norm);
                let view_vec = self.config.camera_pos - res.hit_point;
                let r_dot_v = reflect_vec.dot(view_vec.normalized());
                let specular_term = r_dot_v.powf(self.config.specular_shininess);
                let specular_term = if r_dot_v > 0.0 { specular_term } else { 0.0 };

                s_dot_n * self.object.get_color(t) + specular_term * self.config.specular_color
            }
            None => self.config.background_color
        }
    }

    fn camera_ray_dir(x: f64, y: f64, cam_pos: Vec3, look_at: Vec3, zoom: f64, (width, height): (usize, usize)) -> Vec3 {
        let u = -(x as f64 / width as f64 * 2.0 - 1.0);
        let v = y as f64 / height as f64 * 2.0 - 1.0;

        let look_dir = (look_at - cam_pos).normalized();
        let right_vec = Vec3::from((0, -1, 0)).cross(look_dir);
        let down_vec = look_dir.cross(right_vec);

        let zoomed_cam_pos = cam_pos + zoom * look_dir;
        let pix_pos = zoomed_cam_pos + u * right_vec + v * down_vec;
        let dir = pix_pos - cam_pos;
        dir.normalized()
    }

    fn render_to_image(&self, file: &Path, (width, height): (usize, usize), t: f64) {
        let mut image = DynamicImage::new_rgb8(
            width as u32,
            height as u32,
        );

        let mut buf = vec![Rgba([0, 0, 0, 0]); width * height];
        buf.par_iter_mut().enumerate().for_each(|(i, y)| {
            *y = self.send_pixel_ray(
                i,
                (width, height),
                t,
            ).into();
        });

        // copy buffer to image
        for i in 0..buf.len() {
            let x = (i % width) as u32;
            let y = (i / width) as u32;
            image.put_pixel(x, y, buf[i]);
        }

        image.save_with_format(file, ImageFormat::Png);
    }

    pub fn render_images<F: Fn(u32) -> String>(&self, config: ImageRenderConfiguration<F>) {
        let mut t = config.t_start;
        let mut i = 0u32;

        while t < config.t_stop {
            let image_name = (config.image_name)(i);
            let path = Path::new(&image_name);
            self.render_to_image(path, (config.width, config.height), t);

            i += 1;
            t += config.t_step;
            println!("rendered image {}", image_name);
        }
    }
}

pub struct RayMarcherConfig {
    pub camera_pos: Vec3,
    pub look_at: Vec3,
    pub light_pos: Vec3,
    pub background_color: Vec3,
    pub camera_zoom: f64,
    pub anti_aliasing_level: u32,
    pub backplane_positions: Vec3,
    pub specular_shininess: f64,
    pub specular_color: Vec3,
}

impl Default for RayMarcherConfig {
    fn default() -> Self {
        RayMarcherConfig {
            camera_pos: Vec3 { x: 2.0, y: 4.0, z: 4.0 },
            look_at: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            light_pos: Vec3 { x: 2.0, y: 4.0, z: 4.0 },
            background_color: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            camera_zoom: 3.0,
            anti_aliasing_level: 4u32,
            backplane_positions: Vec3 { x: 3.0, y: 3.0, z: 3.0 },
            specular_shininess: 50.0,
            specular_color: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
        }
    }
}

pub struct ImageRenderConfiguration<F: Fn(u32) -> String> {
    pub width: usize,
    pub height: usize,
    pub t_start: f64,
    pub t_stop: f64,
    pub t_step: f64,
    pub image_name: F,
}