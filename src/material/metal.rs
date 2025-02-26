use crate::{colour::Colour, hittable::HitRecord, random::random_unit_vector, util::reflect, Ray};

use super::Material;

pub struct Metal {
    albedo: Colour,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: &Colour, fuzz: f32) -> Self {
        Self {
            albedo: *albedo,
            fuzz: if fuzz > 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = reflected.normalize() + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}
