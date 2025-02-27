use crate::{Point3, Vec3};

#[derive(Debug, Default, Clone, Copy)]
/// Represents a ray of light in 3D space.
pub struct Ray {
    /// The origin point of the ray.
    orig: Point3,
    /// The direction vector of the ray.
    dir: Vec3,
}

impl Ray {
    /// Creates a new `Ray` with the given origin and direction.
    ///
    /// # Parameters
    /// - `origin`: The starting point of the ray.
    /// - `direction`: The direction vector of the ray.
    ///
    /// # Returns
    /// A new `Ray` instance.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::{Ray, Point3, Vec3};
    /// let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    /// dbg!(ray);
    /// ```
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    /// Returns a reference to the origin point of the ray.
    ///
    /// # Returns
    /// A reference to the origin point.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::{Ray, Point3, Vec3};
    /// let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    /// assert_eq!(ray.origin(), &Point3::new(0.0, 0.0, 0.0));
    /// ```
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    /// Returns a reference to the direction vector of the ray.
    ///
    /// # Returns
    /// A reference to the direction vector.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::{Ray, Point3, Vec3};
    /// let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    /// assert_eq!(ray.direction(), &Vec3::new(1.0, 1.0, 0.0));
    /// ```
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    /// Computes the point at a given distance `t` along the ray.
    ///
    /// # Parameters
    /// - `t`: The distance along the ray.
    ///
    /// # Returns
    /// The point at distance `t` along the ray.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::{Ray, Point3, Vec3};
    /// let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    /// assert_eq!(ray.at(0.0), Vec3::new(0.0, 0.0, 0.0));
    /// ```
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at_zero_vec_origin() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
        assert_eq!(ray.at(0.0), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.at(0.5), Vec3::new(0.5, 0.5, 0.0));
        assert_eq!(ray.at(1.0), Vec3::new(1.0, 1.0, 0.0));
        assert_eq!(ray.at(2.0), Vec3::new(2.0, 2.0, 0.0));
        assert_eq!(ray.at(-2.0), Vec3::new(-2.0, -2.0, 0.0));
    }

    #[test]
    fn ray_at_zero_vec_dir() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.at(0.0), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.at(0.5), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.at(1.0), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(ray.at(-1.0), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn ray_at() {
        let ray = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(ray.at(0.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.at(0.5), Vec3::new(1.5, 2.5, 3.5));
        assert_eq!(ray.at(1.0), Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(ray.at(-1.0), Vec3::new(0.0, 1.0, 2.0));
    }
}
