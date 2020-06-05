use crate::vec3::Vec3;
use crate::scene::{SceneVec, Scene};
use lazy_static::lazy_static;
use crate::ray::cast_ray;
use crate::scene_object::Sphere;

const VIEW_PLANE_DIST: f64 = 40.0;
const VIEW_PLANE_WIDTH: f64 = 20.0;
const VIEW_PLANE_HEIGHT: f64 = 20.0;

fn construct_scene() -> SceneVec {
    vec![
        Box::new(Sphere {
            center: (0, 0, 0).into(),
            radius: 0.8,
            color: (1.0, 0.0, 0.0).into(),
        }),
        Box::new(Sphere {
            center: (1, 0, 0).into(),
            radius: 0.3,
            color: (0.0, 1.0, 0.0).into(),
        })
    ]
}


pub struct RayMarcher {
    scene: SceneVec
}

impl RayMarcher {
    const X_MIN: f64 = -VIEW_PLANE_WIDTH * 0.5;
    const X_MAX: f64 = VIEW_PLANE_WIDTH * 0.5;
    const Y_MIN: f64 = -VIEW_PLANE_HEIGHT * 0.5;
    const Y_MAX: f64 = VIEW_PLANE_HEIGHT * 0.5;
    const CELL_WIDTH: f64 = VIEW_PLANE_WIDTH / (super::WIDTH as f64);
    const CELL_HEIGHT: f64 = VIEW_PLANE_HEIGHT / (super::HEIGHT as f64);

    const CAMERA_POS: Vec3 = Vec3 { x: 2.0, y: 1.0, z: 0.5 };
    const LOOK_AT: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    const LIGHT_POS: Vec3 = Vec3 {x: 2.0, y: 1.0, z: 1.0};
    const BG_COLOR: Vec3 = Vec3 {x: 0.0, y: 0.0, z:0.0};

    pub fn new() -> Self {
        RayMarcher { scene: construct_scene() }
    }

    pub fn draw(&self, frame: &mut [u32]) {
        for (i, pix) in frame.iter_mut().enumerate() {
            *pix = self.send_pixel_ray(i).into()
        }
    }

    fn send_pixel_ray(&self, buffer_idx: usize) -> Vec3 {
        let x = buffer_idx % super::WIDTH;
        let y = buffer_idx / super::HEIGHT;

        let ray_dir = Self::camera_ray_dir(x, y, RayMarcher::CAMERA_POS, RayMarcher::LOOK_AT, 1.0);
        self.trace(RayMarcher::CAMERA_POS, ray_dir)
    }

    fn trace(&self, point: Vec3, dir: Vec3) -> Vec3 {
        let res = cast_ray(&self.scene, point, dir);
        match res {
            Some(res) => {
                let norm = self.scene.normal(res.hit_point);
                let s_dot_n = norm.dot((RayMarcher::LIGHT_POS - res.hit_point).normalized());
                s_dot_n * self.scene[res.hit_index].get_color()
            }
            None => RayMarcher::BG_COLOR
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