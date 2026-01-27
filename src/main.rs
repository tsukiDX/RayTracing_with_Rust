mod math;
use math::vector::Point3D;
use math::vector::Vector3;
use math::ray::Ray;

mod simulation;
use simulation::engine::Engine;


const IMAGE_WIDTH : i32 = 256;


fn pixel_main(_ray: &Ray, _x: i32, _y: i32, _u: f32, _v: f32) -> Vector3 {
        
    let pos = Point3D::new(0., 0., -1.);

    let t: Option<f32> = hit_sphere(_ray, &pos, 0.5);
        
    if let Some(v) = t {
        let norm = (_ray.at(v) - pos).normalized();
        (norm + 1.) * 0.5
    } else {
        let unit = _ray.direction().normalized();
        let t = 0.5 * (unit.y + 1.0);

        Vector3::lerp(&Vector3::new(1., 1., 1.), &Vector3::new(0.5, 0.7, 1.0), t)
    }
}

fn main() {
    let engine= &mut Engine::new("output.ppm", IMAGE_WIDTH, 16. / 9., pixel_main);

    eprintln!("[INFO] Simulation started.");
    engine.simulate();
    eprintln!("[INFO] Simulation completed.\n");

    eprintln!("[INFO] Render to PPM started.");
    engine.render();
    eprintln!("[INFO] Render to PPM completed.");
}

fn hit_sphere(ray: &Ray, p: &Point3D, radius: f32) -> Option<f32> {
    let oc = ray.origin() - *p;

    // let t = solve_quadratic(
    //     Vector3::dot(&ray.direction(), &ray.direction()), 
    //     2. * Vector3::dot(&oc, &ray.direction()),
    //     Vector3::dot(&oc, &oc) - radius * radius
    // );

    let t = solve_quadratic_half(
        ray.direction().magnitude_squared(),
        Vector3::dot(&oc, &ray.direction()),
        oc.magnitude_squared() - radius * radius
    );

    if t > 0. {
        Some(t)
    } else {
        None
    }
}

#[allow(dead_code)]
fn solve_quadratic(a: f32, b: f32, c: f32) -> f32 {
    let discriminant = b*b - 4.*a*c;

    if discriminant >= 0. {
        (-b - discriminant.sqrt()) / (2.0 * a)
    } else {
        -1.0
    }
}

fn solve_quadratic_half(a: f32, h: f32, c:f32) -> f32 {
    let discriminant = h * h - a * c;

    if discriminant >= 0. {
        (-h - discriminant.sqrt()) / a
    } else {
        -1.0
    }
}
