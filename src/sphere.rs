use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    Point3,
};
use std::rc::Rc;

/// A sphere in 3D space, defined by its center, radius, and material.
pub struct Sphere {
    /// The center point of the sphere.
    center: Point3,
    /// The radius of the sphere.
    radius: f32,
    /// The material the sphere is made of.
    mat: Rc<dyn Material>,
}

impl Sphere {
    /// Creates a new sphere with the given center, radius, and material.
    ///
    /// # Arguments
    ///
    /// * `center` - A `Point3` representing the center of the sphere.
    /// * `radius` - A `f32` representing the radius of the sphere.
    /// * `mat` - A `Rc<dyn Material>` representing the material of the sphere.
    ///
    /// # Returns
    /// * A new `Sphere` instance.
    pub fn new(center: Point3, radius: f32, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    /// Determines if a ray hits the sphere and updates the hit record accordingly.
    ///
    /// # Arguments
    /// * `r` - A reference to the `Ray` being cast.
    /// * `ray_t` - An `Interval` representing the range of acceptable t values for the ray.
    /// * `rec` - A mutable reference to a `HitRecord` to be updated if the ray hits the sphere.
    ///
    /// # Returns
    /// * `bool` - `true` if the ray hits the sphere, `false` otherwise.
    fn hit(&self, r: &crate::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);

        return true;
    }

    fn mat(&self) -> Option<Rc<dyn Material>> {
        Some(self.mat.clone())
    }
}
