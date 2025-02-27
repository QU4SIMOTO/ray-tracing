use crate::Vec3;

/// Converts degrees to radians.
///
/// # Parameters
/// - `degrees`: The angle in degrees to be converted to radians.
///
/// # Returns
/// The angle in radians.
///
/// # Example
/// ```
/// use std::f32::consts::PI;
/// use ray_tracing::util::degrees_to_radians;
///
/// assert_eq!(degrees_to_radians(90.0), PI / 2.0);
/// ```
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Reflects a vector off a surface with the given normal.
///
/// # Parameters
/// - `v`: The vector to be reflected.
/// - `n`: The normal of the surface.
///
/// # Returns
/// The reflected vector.
///
/// # Example
/// ```
/// use ray_tracing::{util::reflect, Vec3};
/// assert_eq!(
///   reflect(&Vec3::new(1.0, 0.0, 0.0), &Vec3::new(-1.0, -1.0, 0.0)),
///   Vec3::new(-1.0, -2.0, 0.0)
/// )
/// ```
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(*n) * n
}

/// Refracts a vector through a surface with the given normal and refraction index.
///
/// # Parameters
/// - `uv`: The vector to be refracted.
/// - `n`: The normal of the surface.
/// - `etai_over_etat`: The ratio of the indices of refraction.
///
/// # Returns
/// The refracted vector.
///
/// # Example
/// ```
/// use ray_tracing::{util::refract, Vec3};
/// refract(
///   &Vec3::new(1.0, 1.0, 1.0),
///   &Vec3::new(1.0, 1.0, 1.0),
///   0.1
/// );
/// ```
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(-1.0 * uv.dot(*n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

/// Checks if a vector is near zero in all dimensions.
///
/// # Parameters
/// - `v`: The vector to be checked.
///
/// # Returns
/// `true` if the vector is near zero in all dimensions, `false` otherwise.
///
/// # Example
/// ```
/// use ray_tracing::{util::is_vec3_near_zero, Vec3};
/// assert!(is_vec3_near_zero(Vec3::new(0.0, 0.0, 0.0)));
/// assert!(is_vec3_near_zero(Vec3::new(1e-10_f32, 1e-10_f32, 1e-10_f32)));
/// assert!(is_vec3_near_zero(Vec3::new(-1e-10_f32, -1e-10_f32, -1e-10_f32)));
/// assert!(!is_vec3_near_zero(Vec3::new(1.0, 1e-10_f32, 1e-10_f32)));
/// ```
pub fn is_vec3_near_zero(v: Vec3) -> bool {
    let s = 1e-8_f32;
    (v.x.abs() < s) && (v.y.abs() < s) && v.z.abs() < s
}
