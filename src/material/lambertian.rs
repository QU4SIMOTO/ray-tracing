use crate::{
    hittable::HitRecord, random::random_unit_vector, util::is_vec3_near_zero, Colour, Ray,
};

use super::Material;

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: &Colour) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if is_vec3_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
