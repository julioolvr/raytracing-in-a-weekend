use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
            let color = Vector3::new(
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
    let path = Path::new("out/sphere.ppm");
    let mut file = File::create(path)?;

    let width = 800;
    let height = 400;

    writeln!(file, "P3\n{} {}\n255", width, height)?;

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for x in (0..height).rev() {
        for y in 0..width {
            let u = f64::from(y) / f64::from(width);
            let v = f64::from(x) / f64::from(height);

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal.scale(u) + vertical.scale(v)
            );

            let color = color_for(ray);

            let ir = (255.0 * color.x) as u8;
            let ig = (255.0 * color.y) as u8;
            let ib = (255.0 * color.z) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn color_for(ray: Ray) -> Vector3 {
    let sphere_center = Vector3::new(0.0, 0.0, -1.0);

    match hit_sphere(sphere_center, 0.5, ray) {
        Some(t) => {
            // If the ray hits the sphere, then hit_sphere returns at which t that
            // happens. We calculate the point for the ray at t, subtract from the
            // center of the sphere and get the unit vector. That unit vector
            // represents the direction from the center to the surface where the
            // ray hit the sphere.
            let normal = (ray.point_at(t) - sphere_center).unit();

            // We then map that vector to the range 0..1 and map (x,y,z) to (r,g,b)
            Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0).scale(0.5)
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vector3::new(1.0, 1.0, 1.0).scale(1.0 - t) + Vector3::new(0.5, 0.7, 1.0).scale(t)
        }
    }
}

fn hit_sphere(center: Vector3, radius: f64, ray: Ray) -> Option<f64> {
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
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

#[derive(Copy, Clone)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn scale(&self, factor: f64) -> Vector3 {
        Vector3::new(self.x * factor, self.y * factor, self.z * factor)
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn unit(&self) -> Vector3 {
        self.scale(1.0 / self.magnitude())
    }

    fn dot(&self, other: Vector3) -> f64 {
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

#[derive(Copy, Clone)]
struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    fn point_at(&self, t: f64) -> Vector3 {
        self.origin + self.direction.scale(t)
    }
}
