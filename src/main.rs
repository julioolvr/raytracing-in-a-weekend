use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

mod math {
    #[derive(Copy, Clone)]
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Vector3 {
        pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
            Vector3 { x, y, z }
        }

        pub fn scale(&self, factor: f64) -> Vector3 {
            Vector3::new(self.x * factor, self.y * factor, self.z * factor)
        }

        fn magnitude(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }

        pub fn unit(&self) -> Vector3 {
            self.scale(1.0 / self.magnitude())
        }

        pub fn dot(&self, other: Vector3) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
    }

    impl std::ops::Add<Vector3> for Vector3 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl std::ops::Sub<Vector3> for Vector3 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
        }
    }

    impl std::ops::Mul<f64> for Vector3 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self {
            Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }
    }
}

mod raytracer {
    use crate::math;

    #[derive(Copy, Clone)]
    pub struct Ray {
        origin: math::Vector3,
        pub direction: math::Vector3,
    }

    impl Ray {
        fn new(origin: math::Vector3, direction: math::Vector3) -> Ray {
            Ray { origin, direction }
        }

        fn point_at(&self, t: f64) -> math::Vector3 {
            self.origin + self.direction.scale(t)
        }
    }

    pub trait Hitable {
        fn check_hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    }

    pub struct Hit {
        t: f64,
        p: math::Vector3,
        pub normal: math::Vector3,
    }

    impl Hit {
        fn new(t: f64, p: math::Vector3, normal: math::Vector3) -> Hit {
            Hit { t, p, normal }
        }
    }

    pub struct Sphere {
        center: math::Vector3,
        radius: f64,
    }

    impl Sphere {
        pub fn new(center: math::Vector3, radius: f64) -> Sphere {
            Sphere { center, radius }
        }
    }

    impl Hitable for Sphere {
        fn check_hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
            // A ray is a function of the form p(t) = A + B*t, where A is the origin,
            // B the direction. If we consider t a moment in time, the function results
            // in how far the ray has traveled in a specific amount of time.
            //
            // Onto the sphere - the formula for a sphere with center in the origin
            // and radius R is x*x + y*y + z*z = R*R. Any point (x, y, z) that satisfies
            // that equation is on the (surface of the) sphere. With center in C, the
            // equation changes to (x-cx)^2+(y-cy)^2+(z-cz)^2=R^2.
            //
            // To know whether the ray hits the sphere, we need to calculate if there's
            // a t that yields a point that satisfies that equation. That equation results
            // in a quadratic formula that's solved below.
            // If there are two solutions, those are the two t at which the ray enters and
            // exits the sphere. If there's only one, then the ray touches the sphere on a
            // single point right at the surface. If there are no solutions, then the ray
            // does not hit the sphere.
            let oc = ray.origin - self.center;
            let a = ray.direction.dot(ray.direction);
            let b = 2.0 * oc.dot(ray.direction);
            let c = oc.dot(oc) - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant < 0.0 {
                return None;
            }

            let t = (-b - discriminant.sqrt()) / (2.0 * a);

            if t >= t_min && t <= t_max {
                // If the ray hits the sphere, then t represents at which t that
                // happens. We calculate the point for the ray at t, subtract from the
                // center of the sphere and get the unit vector. That unit vector
                // represents the direction from the center to the surface where the
                // ray hit the sphere.
                let normal = (ray.point_at(t) - self.center).unit();
                Some(Hit::new(t, ray.point_at(t), normal))
            } else {
                None
            }
        }
    }

    impl Hitable for Vec<Box<dyn Hitable>> {
        fn check_hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
            let mut closest_hit: Option<Hit> = None;

            for hitable in self {
                let limit = match &closest_hit {
                    Some(hit) => hit.t,
                    None => t_max
                };

                if let Some(hit) = hitable.check_hit(ray, t_min, limit) {
                    closest_hit = Some(hit);
                }
            }

            closest_hit
        }
    }

    pub struct Camera {
        lower_left_corner: math::Vector3,
        horizontal: math::Vector3,
        vertical: math::Vector3,
        origin: math::Vector3
    }

    impl Camera {
        pub fn new(lower_left_corner: math::Vector3, horizontal: math::Vector3, vertical: math::Vector3, origin: math::Vector3) -> Camera {
            Camera { lower_left_corner, horizontal, vertical, origin }
        }

        pub fn get_ray(&self, u: f64, v: f64) -> Ray {
            Ray::new(
                self.origin,
                self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
            )
        }
    }
}
