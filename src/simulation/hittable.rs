use crate::math::vector::Point3D;
use crate::math::vector::Vector3;
use crate::simulation::ray::Ray;

#[allow(dead_code)]
pub struct HitRecord {
    p: Point3D,
    normal: Vector3,
    t: f32,
    front_face: bool
}

#[allow(dead_code)]
pub struct Sphere {
    center: Point3D,
    radius: f32
}

#[allow(dead_code)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

#[allow(dead_code)]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Default for HittableList {
    fn default() -> Self {
        Self { objects: Vec::new()}
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vector3::zero(),
            normal: Vector3::zero(),
            t: 0.,
            front_face: false
        }
    }
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new(p: Point3D, t: f32, ray: &Ray, outward_normal: Vector3) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.;

        Self { 
            p: p, 
            normal:  if front_face {outward_normal} else {-outward_normal},
            t, 
            front_face 
        }
    }

    pub fn p(&self) -> Point3D {
        self.p
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }
}

impl Sphere {
    pub fn new(center: Point3D, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().magnitude_squared();
        let half_b = Vector3::dot(oc, ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;

            let in_range = t_min < t && t < t_max;

            if in_range {
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(p, t, ray, outward_normal));
            }

            let t = (-half_b + root) / a;
            let in_range = t_min < t && t < t_max;

            if in_range {
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(p, t, ray, outward_normal));
            }
        } 

        None
    }
}

#[allow(dead_code)]
impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest: f32 = t_max;
        
        for obj in &self.objects {
            if let Some(h) = obj.hit(ray, t_min, closest) {
                hit = h;
                hit_anything = true;
                closest = hit.t;
            }
        }

        if hit_anything {
            Some(hit)
        } else {
            None
        }
    }
}