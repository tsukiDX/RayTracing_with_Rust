mod math;
use math::vector::Point3D;
use math::vector::Vector3;
use math::ray::Ray;

mod simulation;
use simulation::engine::Engine;


const IMAGE_WIDTH : i32 = 256;


fn pixel_main(ray: &Ray, x: i32, y: i32, u: f32, v: f32) -> Vector3 {
        if hit_sphere(ray, &Point3D::new(0., 0., -1.), 0.5) {
            return Vector3::new(1., 0., 0.)
        }

        let unit = ray.direction().normalized();
        let t = 0.5 * (unit.y + 1.0);

        return Vector3::lerp(&Vector3::new(1., 1., 1.), &Vector3::new(0.5, 0.7, 1.0), t);
    
}

fn main() {
    let engine= &mut Engine::new(IMAGE_WIDTH, 16. / 9., pixel_main);

    eprintln!("[INFO] Simulation started.");
    engine.simulate();
    eprintln!("[INFO] Simulation completed.\n");

    eprintln!("[INFO] Render to PPM started.");
    engine.render();
    eprintln!("[INFO] Render to PPM completed.");
}

fn hit_sphere(ray: &Ray, p: &Point3D, radius: f32) -> bool {
    let oc = ray.origin() - *p;
    let a = Vector3::dot(&ray.direction(), &ray.direction());
    let b = 2. * Vector3::dot(&oc, &ray.direction());
    let c = Vector3::dot(&oc, &oc) - radius * radius;

    b*b - 4.*a*c > 0.
}

