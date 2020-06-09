use crate::scene_object::SceneObject;
use common::vec3::Vec3;

pub struct ZSectioned<O: SceneObject> {
    pub object: O,
    pub z: f64,
}

impl<O: SceneObject> SceneObject for ZSectioned<O> {
    fn distance_to(&self, point: Vec3, t: f64) -> f64 {
        if point.z > self.z {
            let dist_to_cutaway = point.z - self.z;
            let cutaway_projection_point = Vec3::from((point.x, point.y, point.z + self.z));
            let dist_at_cutaway = self.object.distance_to(cutaway_projection_point, t);
            (dist_to_cutaway * dist_at_cutaway + dist_to_cutaway * dist_to_cutaway).sqrt()
        } else {
            self.object.distance_to(point, t)
        }
    }

    fn get_color(&self, t: f64) -> Vec3 {
        self.object.get_color(t)
    }

    fn normal(&self, p: Vec3, t: f64) -> Vec3 {
        if (p.z - self.z).abs() <= 1E-3 || p.z > self.z {
            (0, 1, 0).into()
        } else {
            self.object.normal(p, t)
        }
    }
}