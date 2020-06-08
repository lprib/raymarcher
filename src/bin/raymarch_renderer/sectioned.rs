use crate::scene_object::SceneObject;
use common::vec3::Vec3;

pub struct ZSectioned<T: SceneObject> {
    pub object: T,
    pub z: f64,
}

impl<T: SceneObject> SceneObject for ZSectioned<T> {
    fn distance_to(&self, point: Vec3) -> f64 {
        if point.z > self.z {
            let dist_to_cutaway = point.z - self.z;
            let cutaway_projection_point = Vec3::from((point.x, point.y, point.z + self.z));
            let dist_at_cutaway = self.object.distance_to(cutaway_projection_point);
            (dist_to_cutaway * dist_at_cutaway + dist_to_cutaway * dist_to_cutaway).sqrt()
        } else {
            self.object.distance_to(point)
        }
    }

    fn get_color(&self) -> Vec3 {
        self.object.get_color()
    }

    fn normal(&self, p: Vec3) -> Vec3 {
        if (p.z - self.z).abs() <= 1E-3 || p.z > self.z {
            (0, 1, 0).into()
        } else {
            self.object.normal(p)
        }
    }
}