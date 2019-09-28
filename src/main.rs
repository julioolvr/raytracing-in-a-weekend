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
    let path = Path::new("out/random_spheres.ppm");
    let mut file = File::create(path)?;

    let width = 500;
    let height = 250;
    let samples = 100;

    writeln!(file, "P3\n{} {}\n255", width, height)?;

    let look_from = math::Vector3::new(11.0, 1.8, 3.5);
    let look_at = math::Vector3::new(-1.0, 0.5, 0.0);

    let camera = raytracer::Camera::new(
        look_from,
        look_at,
        math::Vector3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(width) / f64::from(height),
        0.1,
        (look_from - look_at).magnitude(),
    );

    let scene: Vec<Box<dyn raytracer::Hitable>> = random_scene();

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
                return math::Vector3::new(0.0, 0.0, 0.0);
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
            math::Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + math::Vector3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn random_scene() -> Vec<Box<dyn raytracer::Hitable>> {
    let random_spheres = 20 * 20;
    let mut spheres: Vec<Box<dyn raytracer::Hitable>> = Vec::with_capacity(random_spheres + 1 + 3);

    // First a huge "floor" sphere
    spheres.push(Box::new(raytracer::Sphere::new(
        math::Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(raytracer::material::Lambertian::new(math::Vector3::new(
            0.5, 0.5, 0.5,
        ))),
    )));

    // The three "main" ones
    spheres.push(Box::new(raytracer::Sphere::new(
        math::Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(raytracer::material::Dielectric::glass()),
    )));

    spheres.push(Box::new(raytracer::Sphere::new(
        math::Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(raytracer::material::Lambertian::new(math::Vector3::new(
            0.4, 0.2, 0.1,
        ))),
    )));

    spheres.push(Box::new(raytracer::Sphere::new(
        math::Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(raytracer::material::Metal::new(
            math::Vector3::new(0.7, 0.6, 0.5),
            0.0,
        )),
    )));

    // And plenty of random, smaller ones
    for a in -10..10 {
        for b in -10..10 {
            let material_odds: f64 = rand::random();
            let center = math::Vector3::new(
                f64::from(a) + 0.9 * rand::random::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rand::random::<f64>(),
            );

            if (center - math::Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material: Box<dyn raytracer::material::Material> = match material_odds {
                    p if p < 0.8 => {
                        Box::new(raytracer::material::Lambertian::new(math::Vector3::new(
                            rand::random::<f64>() * rand::random::<f64>(),
                            rand::random::<f64>() * rand::random::<f64>(),
                            rand::random::<f64>() * rand::random::<f64>(),
                        )))
                    }
                    p if p < 0.95 => Box::new(raytracer::material::Metal::new(
                        math::Vector3::new(
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>()),
                            0.5 * (1.0 + rand::random::<f64>()),
                        ),
                        0.5 * rand::random::<f64>(),
                    )),
                    _ => Box::new(raytracer::material::Dielectric::glass()),
                };

                spheres.push(Box::new(raytracer::Sphere::new(center, 0.2, material)));
            }
        }
    }

    spheres
}
