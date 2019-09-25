use crate::raytracer::{Ray, Hit};
use crate::math::Vector3;

mod lambertian;
mod metal;
mod dielectric;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;

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
