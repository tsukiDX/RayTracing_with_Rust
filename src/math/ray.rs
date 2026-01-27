use crate::math::vector::Vector3;
use crate::math::vector::Point3D;

pub struct Ray {
    origin: Point3D,
    direction: Vector3
}

impl Ray {
    pub fn new(orig: &Point3D, dir: &Vector3) -> Self {
        Self {
            origin: *orig,
            direction: *dir
        }
    }

    pub fn origin(&self) -> Point3D {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}