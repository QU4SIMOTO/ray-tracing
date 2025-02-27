use std::ops;

/// An interval utility.
///
/// An `Interval` is used to define a range of values, with a minimum and maximum boundary.
/// It includes various utility methods for manipulating and querying the interval.
#[derive(Debug, Clone, Copy)]
pub struct Interval<T = f32>
where
    T: Clone,
{
    /// The minimum value of the interval.
    pub min: T,
    /// The maximum value of the interval.
    pub max: T,
}

impl<T> Interval<T>
where
    T: Clone + Copy,
{
    /// Creates a new `Interval` with the given minimum and maximum values.
    ///
    /// # Parameters
    /// - `min`: The minimum value of the interval.
    /// - `max`: The maximum value of the interval.
    ///
    /// # Returns
    /// A new `Interval` instance with the specified `min` and `max` values.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::interval::Interval;
    ///
    /// let interval = Interval::new(0.0, 10.0);
    /// assert_eq!(interval.min, 0.0);
    /// assert_eq!(interval.max, 10.0);
    /// assert!(interval.contains(5.0));
    /// ```
    pub const fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl<T> Interval<T>
where
    T: PartialOrd + Clone + Copy + ops::Sub,
{
    /// Returns the size (length) of the interval, i.e., the difference between `max` and `min`.
    ///
    /// # Returns
    /// The size of the interval.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::interval::Interval;
    ///
    /// let interval = Interval::new(0.0, 10.0);
    /// assert_eq!(interval.size(), 10.0);
    /// ```
    pub fn size(&self) -> <T as ops::Sub>::Output {
        self.max - self.min
    }

    /// Checks if a given value is within the interval, inclusive.
    ///
    /// # Parameters
    /// - `x`: The value to check.
    ///
    /// # Returns
    /// `true` if `x` is within the interval, i.e., `min <= x <= max`; otherwise, `false`.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::interval::Interval;
    ///
    /// let interval = Interval::new(0.0, 10.0);
    /// assert!(interval.contains(5.0));
    /// assert!(interval.contains(10.0));
    /// assert!(!interval.contains(15.0));
    /// ```
    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if a given value is strictly inside the interval (not including the boundaries).
    ///
    /// # Parameters
    /// - `x`: The value to check.
    ///
    /// # Returns
    /// `true` if `x` is strictly within the interval, i.e., `min < x < max`; otherwise, `false`.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::interval::Interval;
    ///
    /// let interval = Interval::new(0.0, 10.0);
    /// assert!(interval.surrounds(5.0));
    /// assert!(!interval.surrounds(10.0));
    /// assert!(!interval.surrounds(15.0));
    /// ```
    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }

    /// Clamps a value to the interval, returning the value if it is within the interval,
    /// or the closest boundary if it is outside the interval.
    ///
    /// # Parameters
    /// - `x`: The value to clamp.
    ///
    /// # Returns
    /// A value clamped to the interval, either the value itself or one of the boundaries.
    ///
    /// # Example
    /// ```
    /// use ray_tracing::interval::Interval;
    ///
    /// let interval = Interval::new(0.0, 10.0);
    /// assert_eq!(interval.clamp(5.0), 5.0);
    /// assert_eq!(interval.clamp(-5.0), 0.0);
    /// assert_eq!(interval.clamp(15.0), 10.0);
    /// ```
    pub fn clamp(&self, x: T) -> T {
        match x {
            _ if x < self.min => self.min,
            _ if x > self.max => self.max,
            _ => x,
        }
    }
}

/// A constant representing an empty interval in `f32`, where `min` is greater than `max`.
///
/// This interval does not contain any values. It can be used to represent the concept
/// of an "empty" or "nonexistent" interval.
pub const EMPTY: Interval<f32> = Interval::new(std::f32::INFINITY, std::f32::NEG_INFINITY);

/// A constant representing the entire universe of `f32` values, from negative infinity to positive infinity.
///
/// This interval includes all possible values of type `f32`.
pub const UNIVERSE: Interval<f32> = Interval::new(std::f32::NEG_INFINITY, std::f32::INFINITY);
