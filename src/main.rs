use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod math;
mod raytracer;

use crate::raytracer::Hitable;

fn main() -> Result<(), std::io::Error> {
    write_hello_world()?;
    write_sphere()?;
    Ok(())
}

fn write_hello_world() -> Result<(), std::io::Error> {
    let path = Path::new("out/hello_world.ppm");
    let mut file = File::create(path)?;

    let width = 200;
    let height = 100;

    writeln!(file, "P3\n{} {}\n255", width, height)?;

    for x in (0..height).rev() {
        for y in 0..width {
            let color = math::Vector3::new(
                f64::from(y) / f64::from(width),
                f64::from(x) / f64::from(height),
                0.2,
            );

            let ir = (255.0 * color.x) as u8;
            let ig = (255.0 * color.y) as u8;
            let ib = (255.0 * color.z) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn write_sphere() -> Result<(), std::io::Error> {
    let path = Path::new("out/sphere_antialiased.ppm");
    let mut file = File::create(path)?;

    let width = 200;
    let height = 100;
    let samples = 100;

    writeln!(file, "P3\n{} {}\n255", width, height)?;

    let camera = raytracer::Camera::new(
        math::Vector3::new(-2.0, -1.0, -1.0),
        math::Vector3::new(4.0, 0.0, 0.0),
        math::Vector3::new(0.0, 2.0, 0.0),
        math::Vector3::new(0.0, 0.0, 0.0)
    );

    let scene: Vec<Box<dyn raytracer::Hitable>> = vec![
        Box::new(raytracer::Sphere::new(math::Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(raytracer::Sphere::new(math::Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    for x in (0..height).rev() {
        for y in 0..width {
            let mut color = math::Vector3::new(0.0, 0.0, 0.0);

            for _ in 0..samples {
                // x: 8, y: 12
                let u = (f64::from(y) + rand::random::<f64>()) / f64::from(width);
                let v = (f64::from(x) + rand::random::<f64>()) / f64::from(height);

                let ray = camera.get_ray(u, v);

                color = color + color_for(ray, &scene);
            }

            color = color.scale(1.0 / f64::from(samples));

            let ir = (255.0 * color.x) as u8;
            let ig = (255.0 * color.y) as u8;
            let ib = (255.0 * color.z) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn color_for(ray: raytracer::Ray, scene: &Vec<Box<dyn raytracer::Hitable>>) -> math::Vector3 {
    match scene.check_hit(ray, 0.0, std::f64::MAX) {
        Some(hit) => {
            let normal = hit.normal;
            math::Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0).scale(0.5)
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            math::Vector3::new(1.0, 1.0, 1.0).scale(1.0 - t) + math::Vector3::new(0.5, 0.7, 1.0).scale(t)
        }
    }
}
