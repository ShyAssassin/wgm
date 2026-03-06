use core::fmt;
use super::Vec3;
use crate::units::Rad;
use core::ops::{Index, IndexMut};
use core::ops::{Add, Sub, Mul, Div, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use num_traits::{Float, One, Zero, float::FloatCore, Signed, NumCast};

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
        return Self { x, y }
    }

    /// Create a new `Vec2<T>` with all components set to the same value.
    pub const fn splat(v: T) -> Self
    where T: Copy {
        return Self::new(v, v)
    }

    /// A `Vec2<T>` with all components set to zero.
    pub fn zero() -> Self
    where T: Zero {
        return Self::new(T::zero(), T::zero())
    }

    /// A `Vec2<T>` with all components set to one.
    pub fn one() -> Self
    where T: One {
        return Self::new(T::one(), T::one())
    }

    /// A `Vec2<T>` with the x component set to one and all others to zero.
    pub fn unit_x() -> Self
    where T: Zero + One {
        return Self::new(T::one(), T::zero())
    }

    /// A `Vec2<T>` with the y component set to one and all others to zero.
    pub fn unit_y() -> Self
    where T: Zero + One {
        return Self::new(T::zero(), T::one())
    }

    /// Extend a `Vec2<T>` into a `Vec3<T>` by adding a z component.
    pub fn extend(self, z: T) -> Vec3<T> {
        return Vec3::new(self.x, self.y, z)
    }

    /// Truncate a `Vec2<T>` into a scalar by dropping the y component.
    pub fn truncate(self) -> T {
        return self.x
    }

    /// Return the length of the vector.
    pub fn length(self) -> T
    where T: Float {
        return self.length_squared().sqrt()
    }

    /// Return the squared length of the vector.
    pub fn length_squared(self) -> T
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return self.x * self.x + self.y * self.y
    }

    /// Calculate the distance between `self` and `other`.
    pub fn distance(self, other: Vec2<T>) -> T
    where T: Float {
        return (self - other).length()
    }

    /// Calculate the squared distance between `self` and `other`.
    pub fn distance_squared(self, other: Vec2<T>) -> T
    where T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T> {
        return (self - other).length_squared()
    }

    /// Normalize `self` to unit length.
    pub fn normalize(self) -> Self
    where T: Float {
        let len = self.length();
        if len.is_zero() {
            return self
        }
        return self / len
    }

    /// Normalize `self` within a range of `lower` and `upper`.
    pub fn normalize_between(self, lower: T, upper: T) -> Self
    where T: Float {
        return self.normalize() * (upper - lower) + lower
    }

    /// Compute the dot product of `self` and `rhs`.
    pub fn dot(self, rhs: Vec2<T>) -> T
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Compute the perpendicular vector (rotated 90 degrees counter-clockwise).
    pub fn perp(self) -> Self
    where T: Neg<Output = T> {
        return Self::new(-self.y, self.x)
    }

    /// Compute the perpendicular dot product of `self` and `rhs`.
    pub fn perp_dot(self, rhs: Vec2<T>) -> T
    where T: Copy + Mul<Output = T> + Sub<Output = T> {
        return self.x * rhs.y - self.y * rhs.x
    }

    /// Compute the angle between `self` and `other`, returned as [`Rad<T>`].
    pub fn angle_between(self, other: Vec2<T>) -> Rad<T>
    where T: Float {
        let denom = (self.length_squared() * other.length_squared()).sqrt();
        if denom.is_zero() {
            return Rad(T::zero())
        }
        return Rad((self.dot(other) / denom).acos())
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise.
    pub fn mul_add(self, a: Vec2<T>, b: Vec2<T>) -> Self
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Self::new(self.x * a.x + b.x, self.y * a.y + b.y)
    }

    /// Linearly interpolate between `self` and `other` by `t`.
    pub fn lerp(self, other: Vec2<T>, t: T) -> Self
    where T: Copy + One + Sub<Output = T> + Add<Output = T> + Mul<Output = T> {
        return self * (T::one() - t) + other * t
    }

    /// Return the component-wise minimum of `self` and `other`.
    pub fn min(self, other: Vec2<T>) -> Self
    where T: PartialOrd + Copy {
        return Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
        }
    }

    /// Return the component-wise maximum of `self` and `other`.
    pub fn max(self, other: Vec2<T>) -> Self
    where T: PartialOrd + Copy {
        return Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
        }
    }

    /// Clamp `self` between `min` and `max` component-wise.
    pub fn clamp(self, min: Vec2<T>, max: Vec2<T>) -> Self
    where T: PartialOrd + Copy {
        return self.max(min).min(max)
    }

    /// Returns a vector with the absolute value of each component.
    pub fn abs(self) -> Self
    where T: Signed {
        return Self::new(self.x.abs(), self.y.abs())
    }

    /// Returns a vector with the sign of each component.
    pub fn signum(self) -> Self
    where T: Signed {
        return Self::new(self.x.signum(), self.y.signum())
    }

    /// Returns a vector with the reciprocal of each component.
    pub fn recip(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.recip(), self.y.recip())
    }

    /// Returns a vector with the floor of each component.
    pub fn floor(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.floor(), self.y.floor())
    }

    /// Returns a vector with the ceil of each component.
    pub fn ceil(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.ceil(), self.y.ceil())
    }

    /// Returns a vector with the round of each component.
    pub fn round(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.round(), self.y.round())
    }

    /// Returns a vector with the fractional part of each component.
    pub fn fract(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.fract(), self.y.fract())
    }

    /// Returns a vector with each component raised to the given power.
    pub fn powf(self, n: T) -> Self
    where T: Float {
        return Self::new(self.x.powf(n), self.y.powf(n))
    }

    /// Returns true if any component is NaN.
    pub fn is_nan(self) -> bool
    where T: FloatCore {
        return self.x.is_nan() || self.y.is_nan()
    }

    /// Returns true if all components are finite.
    pub fn is_finite(self) -> bool
    where T: FloatCore {
        return self.x.is_finite() && self.y.is_finite()
    }

    /// Returns true if any component is infinite.
    pub fn is_infinite(self) -> bool
    where T: FloatCore {
        return self.x.is_infinite() || self.y.is_infinite()
    }

    /// Returns the sum of all components.
    pub fn sum(self) -> T
    where T: Add<Output = T> {
        return self.x + self.y
    }

    /// Returns the product of all components.
    pub fn product(self) -> T
    where T: Mul<Output = T> {
        return self.x * self.y
    }

    /// Returns the minimum element.
    pub fn min_element(self) -> T
    where T: PartialOrd {
        return if self.x < self.y { self.x } else { self.y }
    }

    /// Returns the maximum element.
    pub fn max_element(self) -> T
    where T: PartialOrd {
        return if self.x > self.y { self.x } else { self.y }
    }

    /// Returns true if any component is zero.
    pub fn any_zero(self) -> bool
    where T: PartialEq + Zero {
        return self.x.is_zero() || self.y.is_zero()
    }

    /// Returns true if all components are zero.
    pub fn all_zero(self) -> bool
    where T: PartialEq + Zero {
        return self.x.is_zero() && self.y.is_zero()
    }

    /// Reflects the vector about a normal.
    pub fn reflect(self, normal: Vec2<T>) -> Self
    where T: Copy + NumCast + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        let two: T = NumCast::from(2.0f64).unwrap();
        return self - normal * (two * self.dot(normal))
    }

    /// Projects this vector onto another.
    pub fn project_onto(self, other: Vec2<T>) -> Self
    where T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Zero + PartialEq {
        let denom = other.length_squared();
        if denom.is_zero() {
            return Self::zero()
        }
        return other * (self.dot(other) / denom)
    }

    /// Rejects this vector from another (component perpendicular to `other`).
    pub fn reject_from(self, other: Vec2<T>) -> Self
    where T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Zero + PartialEq {
        return self - self.project_onto(other)
    }

    /// Rotates this vector by the given angle.
    pub fn rotate(self, angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        return Self::new(
            self.x * c - self.y * s,
            self.x * s + self.y * c,
        )
    }

    /// Returns the angle of this vector from the positive x-axis, as [`Rad<T>`].
    pub fn angle(self) -> Rad<T>
    where T: Float {
        return Rad(self.y.atan2(self.x))
    }

    /// Converts this vector to an array.
    pub fn to_array(self) -> [T; 2] {
        return [self.x, self.y]
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => return &self.x,
            1 => return &self.y,
            _ => panic!("index out of bounds: Vec2 has 2 components but index is {}", index),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => return &mut self.x,
            1 => return &mut self.y,
            _ => panic!("index out of bounds: Vec2 has 2 components but index is {}", index),
        }
    }
}

// Vec2<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Vec2::new(self.x * rhs, self.y * rhs)
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
        return Vec2::new(self.x / rhs, self.y / rhs)
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
        return Vec2::new(self.x + rhs, self.y + rhs)
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
        return Vec2::new(self.x - rhs, self.y - rhs)
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
        return Vec2::new(self.x * rhs.x, self.y * rhs.y)
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
        return Vec2::new(self.x / rhs.x, self.y / rhs.y)
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
        return Vec2::new(self.x + rhs.x, self.y + rhs.y)
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
        return Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// Vec2<T> -= Vec2<T>
impl<T: Copy + Sub<Output = T>> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

// -Vec2<T>
impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        return Vec2::new(-self.x, -self.y)
    }
}

// T -> Vec2<T>
impl<T: Copy> From<T> for Vec2<T> {
    fn from(v: T) -> Self {
        return Self::new(v, v)
    }
}

// [T; 2] -> Vec2<T>
impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        return Self::new(x, y)
    }
}

// (T, T) -> Vec2<T>
impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        return Self::new(x, y)
    }
}

// Vec2<T> -> [T; 2]
impl<T> From<Vec2<T>> for [T; 2] {
    fn from(v: Vec2<T>) -> Self {
        return [v.x, v.y]
    }
}

// Vec2<T> -> (T, T)
impl<T> From<Vec2<T>> for (T, T) {
    fn from(v: Vec2<T>) -> Self {
        return (v.x, v.y)
    }
}
