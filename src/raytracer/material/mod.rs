use crate::raytracer::{Ray, Hit};
use crate::math::Vector3;

mod lambertian;
mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;

pub struct ScatteredHit {
    pub ray: Ray,
    pub attenuation: Vector3,
}

impl ScatteredHit {
    fn new(ray: Ray, attenuation: Vector3) -> ScatteredHit {
        ScatteredHit { ray, attenuation }
    }
}

pub trait Material {
    fn scatter(&self, hit: &Hit, ray: &Ray) -> Option<ScatteredHit>;
}
