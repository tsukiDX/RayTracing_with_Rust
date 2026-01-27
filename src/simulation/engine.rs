use core::slice;
use std::fs::File;
use std::io::{self, Write};

use crate::math::ray::Ray;
use crate::simulation::result_image::{RGB256, ResultImage};
use crate::math::vector::{Point3D, Vector3};

pub struct Engine<F> 
where 
    F: Fn(&Ray,i32, i32, f32, f32) -> Vector3,
{
    image: ResultImage,
    simulate: F,
    export_file_name: String
}

impl<F> Engine<F> 
where 
    F: Fn(&Ray,i32, i32, f32, f32) -> Vector3,
{
    pub fn new(file_name: &str, width_resolution: i32, aspect_ratio: f32, simulate: F) -> Self {
        let width = width_resolution;
        let height = ((width_resolution as f32) / aspect_ratio) as i32;
        Self {
            image: ResultImage::new(width, height),
            simulate,
            export_file_name: String::from(file_name)
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

                self.image.pixels[index] = RGB256 {
                    data: [
                        (color.x * 255.999) as u8,
                        (color.y * 255.999) as u8,
                        (color.z * 255.999) as u8
                    ]
                };
            }
        }
    }

    pub fn render(&self) -> io::Result<()> {

        let file = File::create(self.export_file_name.as_str())?;
        let mut o = std::io::BufWriter::new(file);

        {
            let img = &self.image;
            write!(o, "P6\n{} {}\n255\n", img.width(), img.height())?;
        }

        {
            let pixels = &self.image.pixels;
            
            let ptr = pixels.as_ptr() as *const u8;
            let len = pixels.len() * 3;

            let bytes: &[u8] = unsafe {
                slice::from_raw_parts(ptr, len)
            };

            o.write_all(bytes)?;
        }

        Ok(())
    }
}