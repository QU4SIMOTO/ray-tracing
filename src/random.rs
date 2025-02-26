use crate::Vec3;
use rand::prelude::*;

pub fn random_f32() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(0.0..=1.0)
}

pub fn random_f32_bounded(min: f32, max: f32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_f32(), random_f32(), random_f32())
}

pub fn random_vec3_bounded(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_f32_bounded(min, max),
        random_f32_bounded(min, max),
        random_f32_bounded(min, max),
    )
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_vec3_bounded(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160_f64 < lensq as f64 && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(*normal) > 0.0 {
        // In the same hemisphere as the normal
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_f32_bounded(-1.0, 1.0),
            random_f32_bounded(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
