use core::{fmt, ops};

use crate::interval::Interval;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    e: [f32; 3],
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> &f32 {
        &self.e[0]
    }

    pub fn y(&self) -> &f32 {
        &self.e[1]
    }

    pub fn z(&self) -> &f32 {
        &self.e[2]
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Self {
        Self::new(
            rand::random_range(0.0..=1.0),
            rand::random_range(0.0..=1.0),
            rand::random_range(0.0..=1.0),
        )
    }

    pub fn random_in_range(min: f32, max: f32) -> Self {
        Self::new(
            rand::random_range(min..=max),
            rand::random_range(min..=max),
            rand::random_range(min..=max),
        )
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            let lensq = p.length_squared();

            // Maybe black hole the zero vector?
            if 1e-10 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add<Self> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign<Self> for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Self> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign<Self> for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

pub type Point3 = Vector3;

pub type Color = Vector3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl fmt::Display for Wrapper<&Color> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.0.x();
        let g = self.0.y();
        let b = self.0.z();

        const INTENSITY: Interval = Interval::new(0.000, 0.999);
        let rbytes = (256.0 * INTENSITY.clamp(*r)) as i32;
        let gbytes = (256.0 * INTENSITY.clamp(*g)) as i32;
        let bbytes = (256.0 * INTENSITY.clamp(*b)) as i32;

        write!(f, "{} {} {}", rbytes, gbytes, bbytes)
    }
}
