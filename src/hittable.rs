use crate::{interval::Interval, material::Material, Ray};
use std::rc::Rc;

mod hit_record;
mod hittable_list;

pub use hit_record::HitRecord;
pub use hittable_list::HittableList;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn mat(&self) -> Option<Rc<dyn Material>>;
}
