use crate::math::Vector3;
use crate::raytracer::Ray;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        view_up: Vector3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        // u, v and w will form the coordinate system for the camera
        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let lens_radius = aperture / 2.0;

        // This is the angle visible to the camera, top to bottom
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        // The aspect ratio will be how much wider than taller is the viewport
        let half_width = half_height * aspect_ratio;

        let origin = look_from;

        let lower_left_corner = origin
            - half_width * focus_distance * u
            - half_height * focus_distance * v
            - w * focus_distance;
        let horizontal = 2.0 * half_width * u * focus_distance;
        let vertical = 2.0 * half_height * v * focus_distance;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let point_in_lens = self.lens_radius * random_in_unit_disk();
        let offset = self.u * point_in_lens.x + self.v * point_in_lens.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vector3 {
    loop {
        let p =
            Vector3::new(rand::random(), rand::random(), 0.0) * 2.0 - Vector3::new(1.0, 1.0, 0.0);

        if p.squared_length() <= 1.0 {
            return p;
        }
    }
}
