mod math;
use math::vector::Vector3;
use math::vector::Point3D;

mod simulation;
use simulation::ray::Ray;
use simulation::engine::Engine;
use simulation::hittable::HittableList;
use simulation::hittable::Sphere;

use crate::math::vector::Vector2;
use crate::simulation::hittable::Hittable;


const IMAGE_WIDTH : i32 = 1920;


fn pixel_main(_ray: &Ray, _world: &HittableList, _x: i32, _y: i32, _u: f32, _v: f32) -> Vector3 {
    // let t = _world.hit(_ray, 0., 1000.0);

    // if let Some(v) = t {
    //     (v.normal() + 1.) * 0.5
    // } else {
    //     let unit = _ray.direction().normalized();
    //     let t = 0.5 * (unit.y + 1.0);

    //     Vector3::lerp(Vector3{x: 1., y: 1., z: 1.}, Vector3{x: 0.5, y: 0.7, z: 1.}, t)
    // }

    let t = 10.0;
    
    let v = math::noise::hash::Perlin::rand21(Vector2 {x: _u * t, y: _v * t});

    Vector3 { x: v, y: v, z: v }
}

fn main() {
    let engine= &mut Engine::new("output.ppm", IMAGE_WIDTH, 16. / 9., pixel_main);

    engine.world().add(Sphere::new(Point3D::new(0., 0., -1.), 0.5));
    engine.world().add(Sphere::new(Point3D::new(0.,-100.5,-1.), 100.));

    eprintln!("[INFO] Simulation started.");
    engine.simulate();
    eprintln!("[INFO] Simulation completed.\n");

    eprintln!("[INFO] Render to PPM started.");
    engine.render();
    eprintln!("[INFO] Render to PPM completed.");
}