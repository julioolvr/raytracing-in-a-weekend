use crate::raytracer::{Ray, Hit, Material};
use crate::raytracer::material::ScatteredHit;
use crate::math::{Vector3, random_in_unit_sphere};

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit: &Hit, _ray: &Ray) -> Option<ScatteredHit> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        Some(ScatteredHit::new(scattered, self.albedo))
    }
}
