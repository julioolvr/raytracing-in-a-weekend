use crate::math::Vector3;
use crate::raytracer::Ray;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3
}

impl Camera {
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        view_up: Vector3,
        vertical_fov: f64,
        aspect_ratio: f64
    ) -> Camera {
        // u, v and w will form the coordinate system for the camera
        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        // This is the angle visible to the camera, top to bottom
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        // The aspect ratio will be how much wider than taller is the viewport
        let half_width = half_height * aspect_ratio;

        let origin = look_from;

        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera { lower_left_corner, horizontal, vertical, origin }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
        )
    }
}
