pub use glam::Vec3;
pub type Point3 = Vec3;

mod ray;
pub use ray::Ray;

pub mod camera;
pub mod colour;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod random;
pub mod sphere;
pub mod util;
