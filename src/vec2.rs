use super::Vec3;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A 2-dimensional vector of type T.
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    /// Create a new `Vec2<T>` from the given x and y values.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Create a new `Vec2<T>` with all components set to the same value.
    pub const fn splat(v: T) -> Self
    where T: Copy {
        Self::new(v, v)
    }

    /// Extend a `Vec2<T>` into a `Vec3<T>` by adding a z component.
    pub fn extend(self, z: T) -> Vec3<T> {
        Vec3::new(self.x, self.y, z)
    }

    /// Truncate a `Vec2<T>` into a scalar by dropping the y component.
    pub fn truncate(self) -> T {
        self.x
    }
}

// Vec2<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

// Vec2<T> *= T
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

// Vec2<T> / T
impl<T: Copy + Div<Output = T>> Div<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

// Vec2<T> /= T
impl<T: Copy + Div<Output = T>> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

// Vec2<T> + T
impl<T: Copy + Add<Output = T>> Add<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

// Vec2<T> += T
impl<T: Copy + Add<Output = T>> AddAssign<T> for Vec2<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
    }
}

// Vec2<T> - T
impl<T: Copy + Sub<Output = T>> Sub<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec2::new(self.x - rhs, self.y - rhs)
    }
}

// Vec2<T> -= T
impl<T: Copy + Sub<Output = T>> SubAssign<T> for Vec2<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
    }
}

// Vec2<T> * Vec2<T>
impl<T: Copy + Mul<Output = T>> Mul for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

// Vec2<T> *= Vec2<T>
impl<T: Copy + Mul<Output = T>> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

// Vec2<T> / Vec2<T>
impl<T: Copy + Div<Output = T>> Div for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

// Vec2<T> /= Vec2<T>
impl<T: Copy + Div<Output = T>> DivAssign for Vec2<T> {
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

// Vec2<T> + Vec2<T>
impl<T: Copy + Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// Vec2<T> += Vec2<T>
impl<T: Copy + Add<Output = T>> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

// Vec2<T> - Vec2<T>
impl<T: Copy + Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// Vec2<T> -= Vec2<T>
impl<T: Copy + Sub<Output = T>> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

// T -> Vec2<T>
impl<T: Copy> From<T> for Vec2<T>{
    fn from(v: T) -> Self {
        Self::new(v, v)
    }
}

// [T; 2] -> Vec2<T>
impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

// (T, T) -> Vec2<T>
impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

// Vec2<T> -> [T; 2]
impl<T> From<Vec2<T>> for [T; 2] {
    fn from(v: Vec2<T>) -> Self {
        [v.x, v.y]
    }
}

// Vec2<T> -> (T, T)
impl<T> From<Vec2<T>> for (T, T) {
    fn from(v: Vec2<T>) -> Self {
        (v.x, v.y)
    }
}
