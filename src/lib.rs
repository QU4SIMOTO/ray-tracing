pub use glam::Vec3;
pub type Point3 = Vec3;

mod ray;
pub use ray::Ray;

pub mod camera;
pub mod colour;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod sphere;

use rand::prelude::*;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

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

pub fn is_vec3_near_zero(v: Vec3) -> bool {
    let s = 1e-8_f32;
    (v.x.abs() < s) && (v.y.abs() < s) && v.z.abs() < s
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

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(*n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(-1.0 * uv.dot(*n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}
