use crate::math::vector::{Point3D, Vector3};
use crate::simulation::ray::Ray;

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {

        let viewport_height: f32 = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = Point3D::new(0., 0., 0.);
        let horizontal= Vector3::right() * viewport_width;
        let vertical = Vector3::up() * viewport_height;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vector3::forward() * focal_length;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(&self.origin, &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin))
    }
}