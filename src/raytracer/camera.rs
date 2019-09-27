use crate::math::Vector3;
use crate::raytracer::Ray;
use std::f64::consts::PI;

pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3
}

impl Camera {
    pub fn new(vertical_fov: f64, aspect_ratio: f64) -> Camera {
        // This is the angle visible to the camera, top to bottom
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        // The aspect ratio will be how much wider than taller is the viewport
        let half_width = half_height * aspect_ratio;

        let lower_left_corner = Vector3::new(-half_width, -half_height, -1.0);
        let horizontal = Vector3::new(2.0 * half_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, 2.0 * half_height, 0.0);
        let origin = Vector3::new(0.0, 0.0, 0.0);

        Camera { lower_left_corner, horizontal, vertical, origin }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
        )
    }
}
