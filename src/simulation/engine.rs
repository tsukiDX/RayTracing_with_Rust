use core::slice;
use std::fs::File;
use std::io::{self, Write};

use crate::simulation::ray::Ray;
use crate::simulation::hittable::{HittableList};
use crate::simulation::result_image::{RGB256, ResultImage};
use crate::math::vector::{Color, Vector2, Vector3};
use crate::simulation::camera::Camera;

use crate::math::noise::hash::{Vnoise};

pub struct Engine<F> 
where 
    F: Fn(&Ray, &HittableList, i32, i32, f32, f32, i32) -> Color,
{
    image: ResultImage,
    simulate: F,
    export_file_name: String,
    world: HittableList,
    camera: Camera,
    sample_per_pixel: i32,
    trace: i32
}

impl<F> Engine<F> 
where 
    F: Fn(&Ray, &HittableList, i32, i32, f32, f32, i32) -> Color,
{
    pub fn new(file_name: &str, width_resolution: i32, aspect_ratio: f32, simulate: F, sample_per_pixel: i32, trace: i32) -> Self {
        let width = width_resolution;
        let height = ((width_resolution as f32) / aspect_ratio) as i32;
        Self {
            image: ResultImage::new(width, height),
            simulate,
            export_file_name: String::from(file_name),
            world: HittableList::default(),
            camera: Camera::new(aspect_ratio),
            sample_per_pixel,
            trace
        }
    }

    pub fn world(&mut self) -> &mut HittableList
    {
        &mut self.world
    }

    fn execute_pixel(&self, ray: &Ray, world: &HittableList, x: i32, y: i32, u: f32, v: f32, ray_step: i32) -> Color {
        (self.simulate)(ray, world, x, y, u, v, ray_step)
    }

    pub fn simulate(&mut self) {

        let width = self.image.width();
        let height = self.image.height();

        let sample_scale = 1. / self.sample_per_pixel as f32;

        let resw = 1.0 / (self.image.width() - 1) as f32;
        let resh = 1.0 / (self.image.height() - 1) as f32;

        for y in 0..height {
            for x in 0..width {
                let mut pixel_color = Color::new(0., 0., 0.);

                for s in 0..self.sample_per_pixel {
                    let u: f32 = (x as f32 + (Vnoise::rand21(Vector2::new(s as f32, 0.)) - 0.5) * 2.0) * resw;
                    let v: f32 = (y as f32 + (Vnoise::rand21(Vector2::new(0., s as f32)) - 0.5) * 2.0) * resh;

                    let ray: Ray = self.camera.get_ray(u, v);

                    pixel_color += self.execute_pixel(&ray, &self.world, x, y, u, v, self.trace);
                }

                self.image.pixels[(x + y * width) as usize] = RGB256 {
                    data: [
                        (pixel_color.x * 255.999 * sample_scale) as u8,
                        (pixel_color.y * 255.999 * sample_scale) as u8,
                        (pixel_color.z * 255.999 * sample_scale) as u8
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