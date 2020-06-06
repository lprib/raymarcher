use common::vec3::Vec3;

pub trait SceneObject {
    fn distance_to(&self, point: Vec3) -> f64;
    fn get_color(&self) -> Vec3;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Vec3
}

impl SceneObject for Sphere {
    fn distance_to(&self, point: Vec3) -> f64 {
        ((point - self.center).magnitude() - self.radius).abs()
    }

    fn get_color(&self) -> Vec3 {
        self.color
    }
}

pub struct YPlane {

}