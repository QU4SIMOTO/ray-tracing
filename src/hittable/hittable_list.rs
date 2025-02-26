use crate::{hittable::Hittable, interval::Interval, material::Material, Ray};
use std::rc::Rc;

use super::HitRecord;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        temp_rec.mat = rec.mat.clone();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            let ray_t = Interval::new(ray_t.min, closest_so_far);
            if object.hit(r, ray_t, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                std::mem::swap(rec, &mut temp_rec);
                rec.mat = object.mat().clone();
            }
        }
        return hit_anything;
    }

    fn mat(&self) -> Option<Rc<dyn Material>> {
        None
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! hittable_list {
        () => {
            HittableList::default()
        };
        ($($element:expr),+ $(,)*) => {{
            let mut h = HittableList::default();
            $(h.add($element);)*
            h
        }};
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            colour::Colour, hittable::HittableList, material::Lambertian, sphere::Sphere, Point3,
        };
        use std::rc::Rc;

        #[test]
        fn hittable_list_empty() {
            assert_eq!(hittable_list![].len(), 0);
        }

        #[test]
        fn hittable_list_single_element() {
            assert_eq!(
                hittable_list![Rc::new(Sphere::new(
                    Point3::new(-1.0, 0.0, -1.0),
                    1.0,
                    Rc::new(Lambertian::new(&Colour::new(0.0, 0.0, 1.0))),
                ))]
                .len(),
                1
            );
        }

        #[test]
        fn hittable_list_multiple_elements() {
            assert_eq!(
                hittable_list![
                    Rc::new(Sphere::new(
                        Point3::new(-1.0, 0.0, -1.0),
                        1.0,
                        Rc::new(Lambertian::new(&Colour::new(0.0, 0.0, 1.0))),
                    )),
                    Rc::new(Sphere::new(
                        Point3::new(-1.0, 0.0, -1.0),
                        1.0,
                        Rc::new(Lambertian::new(&Colour::new(0.0, 0.0, 1.0))),
                    ))
                ]
                .len(),
                2
            );
        }

        #[test]
        fn hittable_list_trailing() {
            assert_eq!(
                hittable_list![
                    Rc::new(Sphere::new(
                        Point3::new(-1.0, 0.0, -1.0),
                        1.0,
                        Rc::new(Lambertian::new(&Colour::new(0.0, 0.0, 1.0))),
                    )),
                    Rc::new(Sphere::new(
                        Point3::new(-1.0, 0.0, -1.0),
                        1.0,
                        Rc::new(Lambertian::new(&Colour::new(0.0, 0.0, 1.0))),
                    )),
                ]
                .len(),
                2
            );
        }
    }
}
