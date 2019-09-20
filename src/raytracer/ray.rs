use crate::math::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> Vector3 {
        self.origin + self.direction.scale(t)
    }
}
