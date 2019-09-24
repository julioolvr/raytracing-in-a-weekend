use crate::raytracer::{Ray, Hit};
use crate::math::Vector3;

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

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit: &Hit) -> ScatteredHit {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        ScatteredHit::new(scattered, self.albedo)
    }
}

fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::new(rand::random(), rand::random(), rand::random()).scale(2.0)
            - Vector3::new(1.0, 1.0, 1.0);

        if p.squared_length() <= 1.0 {
            return p;
        }
    }
}
