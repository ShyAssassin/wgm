use num_traits::Float;
use super::{Vec2, Vec3};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A 4-dimensional vector of type T.
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T> {
    /// Create a new `Vec4<T>` from the given x, y, z, and w values.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        return Self { x, y, z, w }
    }

    /// Create a new `Vec4<T>` with all components set to the same value.
    pub const fn splat(v: T) -> Self
    where T: Copy {
        return Self::new(v, v, v, v)
    }

    /// Truncate a `Vec4<T>` into a `Vec3<T>` by dropping the w component.
    pub fn truncate(self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.z)
    }

    /// Return the length of the vector.
    pub fn length(self) -> T
    where T: Float + Mul<Output = T> + Add<Output = T> {
        return self.length_squared().sqrt()
    }

    /// Return the squared length of the vector.
    pub fn length_squared(self) -> T
    where T: Float + Mul<Output = T> + Add<Output = T> {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Calculate the distance between `self` and `other`.
    pub fn distance(self, other: Vec4<T>) -> T
    where T: Float + Sub<Output = T> + Mul<Output = T> + Add<Output = T> {
        (self - other).length()
    }

    /// Normalize `self` within a range of 0 to 1.
    pub fn normalize(self) -> Self
    where T: Float + Div<Output = T> {
        return self / self.length();
    }

    /// Normalize the vector within a range of `lower` to `upper`.
    pub fn normalize_between(self, lower: T, upper: T) -> Self
    where T: Float + Div<Output = T> {
        return self.normalize() * (upper - lower) + lower;
    }

    /// Compute the dot product of `self` and `rhs`.
    pub fn dot(self, rhs: Vec4<T>) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        return (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    /// Linearly interpolate between `self` and `other` by `t`.
    pub fn lerp(self, other: Vec4<T>, t: T) -> Self
    where T: Float + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        return self * (T::one() - t) + other * t;
    }

    /// Return the minimum of `self` and `other`.
    pub fn min(self, other: Vec4<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
            z: if self.z < other.z { self.z } else { other.z },
            w: if self.w < other.w { self.w } else { other.w },
        };
    }

    /// Return the maximum of `self` and `other`.
    pub fn max(self, other: Vec4<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
            z: if self.z > other.z { self.z } else { other.z },
            w: if self.w > other.w { self.w } else { other.w },
        };
    }

    /// Clamp `self` between `min` and `max`.
    pub fn clamp(self, min: Vec4<T>, max: Vec4<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x < min.x { min.x } else if self.x > max.x { max.x } else { self.x },
            y: if self.y < min.y { min.y } else if self.y > max.y { max.y } else { self.y },
            z: if self.z < min.z { min.z } else if self.z > max.z { max.z } else { self.z },
            w: if self.w < min.w { min.w } else if self.w > max.w { max.w } else { self.w },
        };
    }

    /// Returns a vector with the absolute value of each component.
    pub fn abs(self) -> Self
    where T: Float {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    /// Returns true if any component is NaN.
    pub fn is_nan(self) -> bool
    where T: Float {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }

    /// Returns true if all components are finite.
    pub fn is_finite(self) -> bool
    where T: Float {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Returns true if any component is infinite.
    pub fn is_infinite(self) -> bool
    where T: Float {
        self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite() || self.w.is_infinite()
    }

    /// Returns the sum of all components.
    pub fn sum(self) -> T
    where T: Add<Output = T> + Copy {
        self.x + self.y + self.z + self.w
    }

    /// Returns the product of all components.
    pub fn product(self) -> T
    where T: Mul<Output = T> + Copy {
        self.x * self.y * self.z * self.w
    }

    /// Returns the minimum element.
    pub fn min_element(self) -> T
    where T: PartialOrd + Copy {
        let mut min = self.x;
        if self.y < min { min = self.y; }
        if self.z < min { min = self.z; }
        if self.w < min { min = self.w; }
        min
    }

    /// Returns the maximum element.
    pub fn max_element(self) -> T
    where T: PartialOrd + Copy {
        let mut max = self.x;
        if self.y > max { max = self.y; }
        if self.z > max { max = self.z; }
        if self.w > max { max = self.w; }
        max
    }

    /// Returns true if any component is zero.
    pub fn any_zero(self) -> bool
    where T: PartialEq + Copy + num_traits::Zero {
        self.x.is_zero() || self.y.is_zero() || self.z.is_zero() || self.w.is_zero()
    }

    /// Returns true if all components are zero.
    pub fn all_zero(self) -> bool
    where T: PartialEq + Copy + num_traits::Zero {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }

    /// Reflects the vector about a normal.
    pub fn reflect(self, normal: Vec4<T>) -> Self
    where T: Float + Mul<Output = T> + Sub<Output = T> + Add<Output = T> {
        self - normal * (T::from(2.0).unwrap() * self.dot(normal))
    }

    /// Projects this vector onto another.
    pub fn project_onto(self, other: Vec4<T>) -> Self
    where T: Float + Mul<Output = T> + Div<Output = T> + Add<Output = T> {
        let denom = other.length_squared();
        if denom.is_zero() {
            Self::splat(T::zero())
        } else {
            other * (self.dot(other) / denom)
        }
    }
}

// Vec4<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

// Vec4<T> *= T
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

// Vec4<T> / T
impl<T: Copy + Div<Output = T>> Div<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

// Vec4<T> /= T
impl<T: Copy + Div<Output = T>> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self.w = self.w / rhs;
    }
}

// Vec4<T> + T
impl<T: Copy + Add<Output = T>> Add<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec4::new(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs)
    }
}

// Vec4<T> += T
impl<T: Copy + Add<Output = T>> AddAssign<T> for Vec4<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
        self.w = self.w + rhs;
    }
}

// Vec4<T> - T
impl<T: Copy + Sub<Output = T>> Sub<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec4::new(self.x - rhs, self.y - rhs, self.z - rhs, self.w - rhs)
    }
}

// Vec4<T> -= T
impl<T: Copy + Sub<Output = T>> SubAssign<T> for Vec4<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
        self.w = self.w - rhs;
    }
}

// Vec4<T> * Vec4<T>
impl<T: Copy + Mul<Output = T>> Mul<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }
}

// Vec4<T> *= Vec4<T>
impl<T: Copy + Mul<Output = T>> MulAssign<Vec4<T>> for Vec4<T> {
    fn mul_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self.w = self.w * rhs.w;
    }
}

// Vec4<T> / Vec4<T>
impl<T: Copy + Div<Output = T>> Div<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn div(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
    }
}

// Vec4<T> /= Vec4<T>
impl<T: Copy + Div<Output = T>> DivAssign<Vec4<T>> for Vec4<T> {
    fn div_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self.w = self.w / rhs.w;
    }
}

// Vec4<T> + Vec4<T>
impl<T: Copy + Add<Output = T>> Add<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

// Vec4<T> += Vec4<T>
impl<T: Copy + Add<Output = T>> AddAssign<Vec4<T>> for Vec4<T> {
    fn add_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

// Vec4<T> - Vec4<T>
impl<T: Copy + Sub<Output = T>> Sub<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

// Vec4<T> -= Vec4<T>
impl<T: Copy + Sub<Output = T>> SubAssign<Vec4<T>> for Vec4<T> {
    fn sub_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

// T -> Vec4<T>
impl<T: Copy> From<T> for Vec4<T>{
    fn from(v: T) -> Self {
        Self::new(v, v, v, v)
    }
}

// [T; 4] -> Vec4<T>
impl<T> From<[T; 4]> for Vec4<T> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        Self::new(x, y, z, w)
    }
}

// [Vec2<T>; 2] -> Vec4<T>
impl<T> From<[Vec2<T>; 2]> for Vec4<T> {
    fn from([v1, v2]: [Vec2<T>; 2]) -> Self {
        Self::new(v1.x, v1.y, v2.x, v2.y)
    }
}

// (T, T, T, T) -> Vec4<T>
impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self::new(x, y, z, w)
    }
}

// (Vec3<T>, T) -> Vec4<T>
impl<T> From<(Vec3<T>, T)> for Vec4<T> {
    fn from((v, t): (Vec3<T>, T)) -> Self {
        Self::new(v.x, v.y, v.z, t)
    }
}

// (T, Vec3<T>) -> Vec4<T>
impl<T> From<(T, Vec3<T>)> for Vec4<T> {
    fn from((t, v): (T, Vec3<T>)) -> Self {
        Self::new(t, v.x, v.y, v.z)
    }
}

// (Vec2<T>, T, T) -> Vec4<T>
impl<T> From<(Vec2<T>, T, T)> for Vec4<T> {
    fn from((v, z, w): (Vec2<T>, T, T)) -> Self {
        Self::new(v.x, v.y, z, w)
    }
}

// (T, T, Vec2<T>) -> Vec4<T>
impl<T> From<(T, T, Vec2<T>)> for Vec4<T> {
    fn from((t1, t2, v): (T, T, Vec2<T>)) -> Self {
        Self::new(t1, t2, v.x, v.y)
    }
}

// (T, Vec2<T>, T) -> Vec4<T>
impl<T> From<(T, Vec2<T>, T)> for Vec4<T> {
    fn from((t1, v, t2): (T, Vec2<T>, T)) -> Self {
        Self::new(t1, v.x, v.y, t2)
    }
}

// (Vec2<T>, Vec2<T>) -> Vec4<T>
impl<T> From<(Vec2<T>, Vec2<T>)> for Vec4<T> {
    fn from((v1, v2): (Vec2<T>, Vec2<T>)) -> Self {
        Self::new(v1.x, v1.y, v2.x, v2.y)
    }
}

// Vec4<T> -> [T; 4]
impl<T> From<Vec4<T>> for [T; 4] {
    fn from(v: Vec4<T>) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

// Vec4<T> -> [Vec2<T>; 2]
impl<T> From<Vec4<T>> for [Vec2<T>; 2] {
    fn from(v: Vec4<T>) -> Self {
        [Vec2::new(v.x, v.y), Vec2::new(v.z, v.w)]
    }
}

// Vec4<T> -> (T, T, T, T)
impl<T> From<Vec4<T>> for (T, T, T, T) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

// Vec4<T> -> (Vec2<T>, Vec2<T>)
impl<T> From<Vec4<T>> for (Vec2<T>, Vec2<T>) {
    fn from(v: Vec4<T>) -> Self {
        (Vec2::new(v.x, v.y), Vec2::new(v.z, v.w))
    }
}

// Vec4<T> -> (T, T, Vec2<T>)
impl<T> From<Vec4<T>> for (T, T, Vec2<T>) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, v.y, Vec2::new(v.z, v.w))
    }
}

// Vec4<T> -> (Vec2<T>, T, T)
impl<T> From<Vec4<T>> for (Vec2<T>, T, T) {
    fn from(v: Vec4<T>) -> Self {
        (Vec2::new(v.x, v.y), v.z, v.w)
    }
}

// Vec4<T> -> (T, Vec2<T>, T)
impl<T> From<Vec4<T>> for (T, Vec2<T>, T) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, Vec2::new(v.y, v.z), v.w)
    }
}

// Vec4<T> -> (Vec3<T>, T)
impl<T> From<Vec4<T>> for (Vec3<T>, T) {
    fn from(v: Vec4<T>) -> Self {
        (Vec3::new(v.x, v.y, v.z), v.w)
    }
}

// Vec4<T> -> (T, Vec3<T>)
impl<T> From<Vec4<T>> for (T, Vec3<T>) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, Vec3::new(v.y, v.z, v.w))
    }
}
