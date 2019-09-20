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
