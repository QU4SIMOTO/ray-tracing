use crate::random::random_vec3_bounded;
use crate::{interval::Interval, random::random_vec3, Vec3};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Colour(Vec3);

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    /// Get the red channel of the colour.
    pub fn r(&self) -> f32 {
        self.0.x
    }

    /// Get the green channel of the colour.
    pub fn g(&self) -> f32 {
        self.0.y
    }

    /// Get the blue channel of the colour.
    pub fn b(&self) -> f32 {
        self.0.z
    }

    /// Generate a random colour.
    pub fn random() -> Self {
        Self(random_vec3())
    }

    /// Generate a random colour with each channel bounded.
    pub fn random_bounded(min: f32, max: f32) -> Self {
        Self(random_vec3_bounded(min, max))
    }

    fn linear_to_gamma(linear_component: f32) -> f32 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Colour> for f32 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Colour {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = self.0 * rhs;
    }
}

impl Mul<&f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<&Colour> for f32 {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Self::Output {
        Colour(rhs.0 * self)
    }
}

impl MulAssign<&f32> for Colour {
    fn mul_assign(&mut self, rhs: &f32) {
        self.0 = self.0 * rhs;
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = Colour::linear_to_gamma(self.r());
        let g = Colour::linear_to_gamma(self.g());
        let b = Colour::linear_to_gamma(self.b());

        // Translate the [0,1] component values to the byte range [0,255].
        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as u8;
        let gbyte = (256.0 * intensity.clamp(g)) as u8;
        let bbyte = (256.0 * intensity.clamp(b)) as u8;

        // Write out the pixel color components.
        write!(f, "{rbyte} {gbyte} {bbyte}\n")
    }
}
