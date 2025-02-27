pub use glam::Vec3;
pub type Point3 = Vec3;

mod colour;
/// Ray of light in 3D space.
mod ray;
mod sphere;

pub use colour::Colour;
pub use ray::Ray;
pub use sphere::Sphere;

pub mod camera;
pub mod hittable;
/// Interval utility.
pub mod interval;
pub mod material;
pub mod random;
/// Various utility functions.
pub mod util;
