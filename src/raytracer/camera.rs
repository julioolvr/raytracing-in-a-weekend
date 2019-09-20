use crate::math::Vector3;
use crate::raytracer::Ray;

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3
}

impl Camera {
    pub fn new(lower_left_corner: Vector3, horizontal: Vector3, vertical: Vector3, origin: Vector3) -> Camera {
        Camera { lower_left_corner, horizontal, vertical, origin }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
        )
    }
}
