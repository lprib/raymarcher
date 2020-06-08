use common::vec3::Vec3;

const EPS: f64 = 1.0E-10;

pub trait SceneObject: Send + Sync {
    fn distance_to(&self, point: Vec3) -> f64;
    fn get_color(&self) -> Vec3;
    fn normal(&self, p: Vec3) -> Vec3 {
        let x_plus = self.distance_to((p.x + EPS, p.y, p.z).into());
        let x_minus = self.distance_to((p.x - EPS, p.y, p.z).into());
        let y_plus = self.distance_to((p.x, p.y + EPS, p.z).into());
        let y_minus = self.distance_to((p.x, p.y - EPS, p.z).into());
        let z_plus = self.distance_to((p.x, p.y, p.z + EPS).into());
        let z_minus = self.distance_to((p.x, p.y, p.z - EPS).into());

        let x = x_plus - x_minus;
        let y = y_plus - y_minus;
        let z = z_plus - z_minus;
        Vec3 { x, y, z }.normalized()
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Vec3,
}

impl SceneObject for Sphere {
    fn distance_to(&self, point: Vec3) -> f64 {
        ((point - self.center).magnitude() - self.radius).abs()
    }

    fn get_color(&self) -> Vec3 {
        self.color
    }
}