use crate::math::ray::Ray;
use crate::simulation::result_image::{ResultImage};
use crate::math::array::Array;
use crate::math::vector::{Point3D, Vector3};

type VInt3 = Array<i32, 3>;

pub struct Engine<F> 
where 
    F: Fn(&Ray,i32, i32, f32, f32) -> Vector3,
{
    image: ResultImage,
    simulate: F
}

impl<F> Engine<F> 
where 
    F: Fn(&Ray,i32, i32, f32, f32) -> Vector3,
{
    pub fn new(width_resolution: i32, aspect_ratio: f32, simulate: F) -> Self {
        let width = width_resolution;
        let height = ((width_resolution as f32) / aspect_ratio) as i32;
        Self {
            image: ResultImage::new(width, height),
            simulate
        }
    }

    fn run(&self, ray: &Ray, x: i32, y: i32, u: f32, v: f32) -> Vector3 {
        (self.simulate)(ray, x, y, u, v)
    }

    pub fn simulate(&mut self) {

        let width = self.image.width();
        let height = self.image.height();

        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = self.image.aspect_ratio() * viewport_height;
        let focal_len = 1.0;

        let origin = Point3D::new(0., 0., 0.);
        let horizontal = Vector3::new(viewport_width, 0., 0.);
        let vertical = Vector3::new(0., -viewport_height, 0.);

        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Vector3::new(0., 0., focal_len);

        for y in 0..height {
            for x in 0..width {

                let u: f32 = (x as f32) / ((self.image.width() - 1) as f32);
                let v: f32 = (y as f32) / ((self.image.height() - 1) as f32);

                let ray = Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v - origin));

                let index = (x + y * width) as usize;
                let color = self.run(&ray, x, y, u, v);

                self.image.pixels[index] = VInt3 {
                    data: [
                        (color.x * 255.999) as i32,
                        (color.y * 255.999) as i32,
                        (color.z * 255.999) as i32
                    ]
                };
            }
        }
    }

    pub fn render(&self) {
        {
            let img = &self.image;
            println!("P3");
            println!("{} {}", img.width(), img.height());
            println!("{}", 255);
        }

        {
            let pixels = &self.image.pixels;
            for p in pixels {
            println!("{} {} {}", p[0], p[1], p[2]);
            }
        }
    }
}