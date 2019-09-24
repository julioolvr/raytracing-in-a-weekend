use crate::raytracer::{Ray, Hit, Material};
use crate::raytracer::material::ScatteredHit;
use crate::math::{Vector3, random_in_unit_sphere};

pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 0.0 {
            0.0
        } else if fuzz > 1.0 {
            1.0
        } else {
            fuzz
        };

        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, hit: &Hit, ray: &Ray) -> Option<ScatteredHit> {
        let reflected = reflect(ray.direction.unit(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());

        if scattered.direction.dot(hit.normal) > 0.0 {
            Some(ScatteredHit::new(scattered, self.albedo))
        } else {
            None
        }
    }
}

fn reflect(vec_in: Vector3, normal: Vector3) -> Vector3 {
    vec_in - 2.0 * vec_in.dot(normal) * normal
}
