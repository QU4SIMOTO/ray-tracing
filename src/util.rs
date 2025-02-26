use crate::Vec3;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
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

pub fn is_vec3_near_zero(v: Vec3) -> bool {
    let s = 1e-8_f32;
    (v.x.abs() < s) && (v.y.abs() < s) && v.z.abs() < s
}
