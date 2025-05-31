use core::f32;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self::default_impl()
    }
}

impl Interval {
    const fn default_impl() -> Self {
        Self::new(-f32::INFINITY, f32::INFINITY)
    }

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn size(&self) -> f32 {
        self.max - self.min
    }

    pub const fn contains(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }

    pub const fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

pub const EMPTY: Interval = Interval::new(f32::INFINITY, -f32::INFINITY);

pub const UNIVERSE: Interval = Interval::default_impl();
