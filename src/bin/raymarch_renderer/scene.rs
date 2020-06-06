use common::vec3::Vec3;
use crate::scene_object::SceneObject;
use crate::raymarcher::RayMarcher;

pub type SceneVec = Vec<Box<dyn SceneObject>>;

const EPS: f64 = 1.0E-8;

pub trait Scene {
    //returns index into scene vector of the closest object, and the distance to that object
    fn distance_to(&self, point: Vec3) -> (usize, f64);
    fn normal(&self, point: Vec3) -> Vec3;
}

impl Scene for SceneVec {
    fn distance_to(&self, point: Vec3) -> (usize, f64) {
        self
            .iter()
            .map(|object| object.distance_to(point))
            .enumerate()
            // only minimize the distance, not the object index:
            .min_by(|a, b|
                a.1.partial_cmp(&b.1)
                    .expect("NaN in distance function")
            )
            .expect("No minimum distance found for distance functions")
    }

    fn normal(&self, p: Vec3) -> Vec3 {
        let (_, x_plus) = self.distance_to((p.x + EPS, p.y, p.z).into());
        let (_, x_minus) = self.distance_to((p.x - EPS, p.y, p.z).into());
        let (_, y_plus) = self.distance_to((p.x, p.y + EPS, p.z).into());
        let (_, y_minus) = self.distance_to((p.x, p.y - EPS, p.z).into());
        let (_, z_plus) = self.distance_to((p.x, p.y, p.z + EPS).into());
        let (_, z_minus) = self.distance_to((p.x, p.y, p.z - EPS).into());

        let x = x_plus - x_minus;
        let y = y_plus - y_minus;
        let z = z_plus - z_minus;
        Vec3 { x, y, z }.normalized()
    }
}