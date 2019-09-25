use crate::raytracer::{Ray, Hit, Material};
use crate::raytracer::material::ScatteredHit;
use crate::math::{Vector3, reflect};

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    pub fn water() -> Dielectric {
        Dielectric::new(1.3)
    }

    pub fn glass() -> Dielectric {
        Dielectric::new(1.5)
    }

    pub fn diamond() -> Dielectric {
        Dielectric::new(1.8)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit: &Hit, ray: &Ray) -> Option<ScatteredHit> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        // `refract` includes a calculation that will refract with some probability, leaving
        // us to `reflect` if the probability said it didn't refract the ray
        if let Some(refracted) = refract(ray.direction, hit.normal, self.refraction_index) {
            Some(ScatteredHit::new(Ray::new(hit.p, refracted), attenuation))
        } else {
            Some(ScatteredHit::new(Ray::new(hit.p, reflect(ray.direction.unit(), hit.normal)), attenuation))
        }
    }
}

fn refract(incident: Vector3, normal: Vector3, refraction_index: f64) -> Option<Vector3> {
    // I wasn't able to figure out refraction from the book, so I'm following the explanation from
    // scratchapixel.com instead:
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel

    // First, let's figure out the ratio between the refraction index of the current medium
    // with respect to the new medium (this material).
    // We assume the current medium is always air, and we assume air has a refraction index of 1.0.
    // The first assumption can be reconsidered to have things like glass under water.

    // We need to know which is the current medium and which is the new medium though - the ray could
    // be *entering* an object with a dielectric material, or it could be *exiting* one after travelling
    // through it.
    // We check the dot product of the incident ray compared to the normal. If it's negative,
    // then the ray is entering the material. If it's positive, it's exiting.
    // In this design, the direction vector of a ray is not ensured to be a unit vector, so...
    let unit = incident.unit();
    let mut dot = unit.dot(normal);
    let refraction_index_ratio = if dot > 0.0 { refraction_index } else { 1.0 / refraction_index };

    // Likewise, all the equations assume the normal points *away* from the plane the incident ray
    // is hitting. But the normal in this function comes from a hit, and objects in this design always
    // report the normal of a hit pointing *outwards*. So if the hit is from inside the object, the
    // normal will be pointing *inside* of the plane hit by the ray, so we need to turn it around:
    let mut normal = normal;

    // Calculate the cosine of the angle between the incident ray and the normal to use in
    // Schlick's approximation later
    let cosine = if dot < 0.0 {
        (-normal).dot(unit).acos()
    } else {
        normal.dot(unit).acos()
    };

    if dot > 0.0 {
        normal = -normal;
        // And now that the normal was reversed, the dot product should be recalculated... but it will
        // be the same thing, just opposite sign, so we do that.
        dot = -dot;
    }

    // For the equation on the explanation above we have n (refraction_index_ratio) and c1 (the dot
    // product). So we need c2, which doesn't have a great meaning as far as I can tell:
    let c2 = 1.0 - refraction_index_ratio.powi(2) * (1.0 - dot.powi(2));

    // If c2 is negative, then there will be no real solution to the equation. In physical terms,
    // this means that there's *total internal reflection* - or in other words, no refraction (and
    // no refracted ray).
    // schlick gives us the probability of the ray being reflected. If the probability is 0.8,
    // then we have a 20% chance of getting a number higher than that by calling rand::random().
    // So if our random number is higher than the chances of reflecting, we refract, otherwise
    // we reflect.
    if c2 > 0.0 && schlick(cosine, refraction_index) < rand::random() {
        Some(refraction_index_ratio * unit + normal * (refraction_index_ratio * dot - c2))
    } else {
        None
    }
}

// Schlick's approximation for the reflection of a ray at angle `cosine` over a surface with the
// given `refraction_index`.
fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
