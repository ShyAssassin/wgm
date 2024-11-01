use num_traits::Float;
use super::{Vec2, Vec4};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A 3-dimensional vector of type T.
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    /// Create a new `Vec2<T>` from the given x, y and z values.
    pub const fn new(x: T, y: T, z: T) -> Self {
        return Self { x, y, z }
    }

    /// Create a new `Vec3<T>` with all elements set to the same value.
    pub const fn splat(v: T) -> Self
    where T: Copy {
        return Self::new(v, v, v)
    }

    /// Extend a `Vec3<T>` into a `Vec4<T>` by adding a w component.
    pub fn extend(self, w: T) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, w)
    }

    /// Truncate a `Vec3<T>` into a `Vec2<T>` by dropping the z component.
    pub fn truncate(self) -> Vec2<T> {
        return Vec2::new(self.x, self.y)
    }

    /// Return the length of the vector.
    pub fn length(self) -> T
    where T: Float + Mul<Output = T> + Add<Output = T> {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize the vector.
    pub fn normalize(self) -> Self
    where T: Float + Div<Output = T> {
        return self / self.length();
    }

    /// Compute the dot product of `self` and `rhs`.
    pub fn dot(self, rhs: Vec3<T>) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        return (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Compute the cross product of `self` and `rhs`.
    pub fn cross(self, rhs: Vec3<T>) -> Vec3<T>
    where T: Copy + Mul<Output = T> + Sub<Output = T> {
        return Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

// Vec3<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Vec3<T> *= T
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

// Vec3<T> / T
impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// Vec3<T> /= T
impl<T: Copy + Div<Output = T>> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

// Vec3<T> + T
impl<T: Copy + Add<Output = T>> Add<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

// Vec3<T> += T
impl<T: Copy + Add<Output = T>> AddAssign<T> for Vec3<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
    }
}

// Vec3<T> - T
impl<T: Copy + Sub<Output = T>> Sub<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

// Vec3<T> -= T
impl<T: Copy + Sub<Output = T>> SubAssign<T> for Vec3<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
    }
}

// Vec3<T> * Vec3<T>
impl<T: Copy + Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

// Vec3<T> *= Vec3<T>
impl<T: Copy + Mul<Output = T>> MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

// Vec3<T> / Vec3<T>
impl<T: Copy + Div<Output = T>> Div<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

// Vec3<T> /= Vec3<T>
impl<T: Copy + Div<Output = T>> DivAssign<Vec3<T>> for Vec3<T> {
    fn div_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}

// Vec3<T> + Vec3<T>
impl<T: Copy + Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Vec3<T> += Vec3<T>
impl<T: Copy + Add<Output = T>> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

// Vec3<T> - Vec3<T>
impl<T: Copy + Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Vec3<T> -= Vec3<T>
impl<T: Copy + Sub<Output = T>> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

// T -> Vec3<T>
impl<T: Copy> From<T> for Vec3<T>{
    fn from(v: T) -> Self {
        Self::new(v, v, v)
    }
}

// [T; 3] -> Vec3<T>
impl<T> From<[T; 3]> for Vec3<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}

// (T, T, T) -> Vec3<T>
impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self::new(x, y, z)
    }
}

// (T, Vec2<T>) -> Vec3<T>
impl<T> From<(T, Vec2<T>)> for Vec3<T> {
    fn from((t, v): (T, Vec2<T>)) -> Self {
        Self::new(t, v.x, v.y)
    }
}

// (Vec2<T>, T) -> Vec3<T>
impl<T> From<(Vec2<T>, T)> for Vec3<T> {
    fn from((v, t): (Vec2<T>, T)) -> Self {
        Self::new(v.x, v.y, t)
    }
}

// Vec3<T> -> [T; 3]
impl<T> From<Vec3<T>> for [T; 3] {
    fn from(v: Vec3<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

// Vec3<T> -> (T, T, T)
impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(v: Vec3<T>) -> Self {
        (v.x, v.y, v.z)
    }
}

// Vec3<T> -> (T, Vec2<T>)
impl<T> From<Vec3<T>> for (T, Vec2<T>) {
    fn from(v: Vec3<T>) -> Self {
        (v.x, Vec2::new(v.y, v.z))
    }
}

// Vec3<T> -> (Vec2<T>, T)
impl<T> From<Vec3<T>> for (Vec2<T>, T) {
    fn from(v: Vec3<T>) -> Self {
        (Vec2::new(v.x, v.y), v.z)
    }
}
