use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod math;
mod raytracer;

fn main() -> Result<(), std::io::Error> {
    write_sphere()?;
    Ok(())
}

fn write_sphere() -> Result<(), std::io::Error> {
    let path = Path::new("out/sphere_camera.ppm");
    let mut file = File::create(path)?;

    let width = 500;
    let height = 250;
    let samples = 100;

    writeln!(file, "P3\n{} {}\n255", width, height)?;

    let camera = raytracer::Camera::new(
        math::Vector3::new(-2.0, 2.0, 1.0),
        math::Vector3::new(0.0, 0.0, -1.0),
        math::Vector3::new(0.0, 1.0, 0.0),
        30.0,
        f64::from(width) / f64::from(height),
    );

    let scene: Vec<Box<dyn raytracer::Hitable>> = vec![
        Box::new(raytracer::Sphere::new(
            math::Vector3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(raytracer::material::Lambertian::new(math::Vector3::new(0.1, 0.2, 0.5)))
        )),
        Box::new(raytracer::Sphere::new(
            math::Vector3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(raytracer::material::Lambertian::new(math::Vector3::new(0.8, 0.8, 0.0)))
        )),
        Box::new(raytracer::Sphere::new(
            math::Vector3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(raytracer::material::Metal::new(math::Vector3::new(0.8, 0.6, 0.2), 0.5))
        )),
        Box::new(raytracer::Sphere::new(
            math::Vector3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(raytracer::material::Dielectric::glass())
        )),
    ];

    for x in (0..height).rev() {
        for y in 0..width {
            let mut color = math::Vector3::new(0.0, 0.0, 0.0);

            for _ in 0..samples {
                let u = (f64::from(y) + rand::random::<f64>()) / f64::from(width);
                let v = (f64::from(x) + rand::random::<f64>()) / f64::from(height);

                let ray = camera.get_ray(u, v);

                color = color + color_for(ray, &scene, 0);
            }

            color = color * (1.0 / f64::from(samples));

            let ir = (255.0 * color.x.sqrt()) as u8;
            let ig = (255.0 * color.y.sqrt()) as u8;
            let ib = (255.0 * color.z.sqrt()) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn color_for(ray: raytracer::Ray, scene: &dyn raytracer::Hitable, depth: usize) -> math::Vector3 {
    match scene.check_hit(ray, 0.0001, std::f64::MAX) {
        Some(hit) => {
            if depth >= 50 {
                return math::Vector3::new(0.0, 0.0, 0.0)
            }

            if let Some(scattered_hit) = hit.material.scatter(&hit, &ray) {
                color_for(scattered_hit.ray, scene, depth + 1) * scattered_hit.attenuation
            } else {
                math::Vector3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            math::Vector3::new(1.0, 1.0, 1.0) * (1.0 - t)
                + math::Vector3::new(0.5, 0.7, 1.0) * t
        }
    }
}
