use crate::raytracer::{Ray, Hit};
use crate::math::Vector3;

mod lambertian;

pub use lambertian::Lambertian;

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
    fn scatter(&self, hit: &Hit) -> ScatteredHit;
}
