mod math;
use math::vector::Vector3;
use math::vector::Point3D;

mod simulation;
use simulation::ray::Ray;
use simulation::engine::Engine;
use simulation::hittable::HittableList;
use simulation::hittable::Sphere;


use crate::math::vector::Color;
use crate::simulation::hittable::Hittable;


const IMAGE_WIDTH : i32 = 256;


fn pixel_main(_ray: &Ray, _world: &HittableList, _x: i32, _y: i32, _u: f32, _v: f32, trace: i32) -> Color {
    let t = _world.hit(_ray, 0., 1000.0);

    if (trace <= 0) {
        return Color::new(0., 0., 0.);
    }

    if let Some(v) = t {
        let target: Point3D = v.p() + v.normal();
        return pixel_main(&Ray::new(&v.p(), &(target - v.p())), _world, _x, _y, _u, _v, trace - 1) * 0.5;
    } else {
        let unit = _ray.direction().normalized();
        let t = 0.5 * (unit.y + 1.0);

        return Vector3::lerp(Vector3{x: 1., y: 1., z: 1.}, Vector3{x: 0.5, y: 0.7, z: 1.}, t)
    }
}

fn main() {
    let engine= &mut Engine::new("output.ppm", IMAGE_WIDTH, 16. / 9., pixel_main, 500, 31);

    engine.world().add(Sphere::new(Point3D::new(0., 0., -1.), 0.5));
    engine.world().add(Sphere::new(Point3D::new(0.,-100.5,-1.), 100.));

    eprintln!("[INFO] Simulation started.");
    engine.simulate();
    eprintln!("[INFO] Simulation completed.\n");

    eprintln!("[INFO] Render to PPM started.");
    engine.render();
    eprintln!("[INFO] Render to PPM completed.");
}