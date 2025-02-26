#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        match x {
            _ if x < self.min => self.min,
            _ if x > self.max => self.max,
            _ => x,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: std::f32::NEG_INFINITY,
            max: std::f32::INFINITY,
        }
    }
}

pub const EMPTY: Interval = Interval::new(std::f32::INFINITY, std::f32::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(std::f32::NEG_INFINITY, std::f32::INFINITY);
