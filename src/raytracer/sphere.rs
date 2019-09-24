use crate::math::Vector3;
use crate::raytracer::{Ray, Hit, Hitable, Material};

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
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
            Some(Hit::new(t, ray.point_at(t), normal, &(*self.material)))
        } else {
            None
        }
    }
}
