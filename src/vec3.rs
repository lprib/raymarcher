use std::ops::{Mul, Neg, Sub, Add};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn from_f64(n: f64) -> Self {
        Vec3 { x: n, y: n, z: n }
    }

    pub fn magnitude(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalized(self) -> Self {
        (1.0 / self.magnitude()) * self
    }

    pub fn reflect(self, norm: Vec3) -> Self {
        let norm = norm.normalized();
        self - 2.0 * self.dot(norm) * norm
    }

    pub fn cross(self, other: Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn refract(self, norm: Vec3, eta: f64) -> Self {
        let k = 1.0 - eta * eta * (1.0 - self.dot(norm) * self.dot(norm));
        if k < 0.0 {
            Default::default()
        } else {
            eta * self - (eta * self.dot(norm) + k.sqrt()) * norm
        }
    }

    pub fn lerp(self, other: Vec3, t: f64) -> Self {
        self + t * (other - self)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vec3 { x, y, z }
    }
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Vec3 { x: x as f64, y: y as f64, z: z as f64 }
    }
}

impl From<Vec3> for u32 {
    fn from(vec: Vec3) -> Self {
        u32::from_be_bytes([
            0,
            to_color_byte(vec.x),
            to_color_byte(vec.y),
            to_color_byte(vec.z)
        ])
    }
}

fn to_color_byte(val: f64) -> u8 {
    let val = clamp(val, 0.0, 1.0);
    (val * 255.0) as u8
}

fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}