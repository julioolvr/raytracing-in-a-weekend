use crate::math::Vector3;
use crate::raytracer::Ray;

pub struct Hit {
    t: f64,
    pub p: Vector3,
    pub normal: Vector3,
}

impl Hit {
    pub fn new(t: f64, p: Vector3, normal: Vector3) -> Hit {
        Hit { t, p, normal }
    }
}

pub trait Hitable {
    fn check_hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
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
