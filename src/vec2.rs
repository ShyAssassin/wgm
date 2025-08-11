use super::Vec3;
use num_traits::Float;
use core::ops::{Add, Sub, Mul, Div};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

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
    where T: Float + Mul<Output = T> + Add<Output = T> {
        return self.length_squared().sqrt()
    }

    /// Return the squared length of the vector.
    pub fn length_squared(self) -> T
    where T: Float + Mul<Output = T> + Add<Output = T> {
        return self.x * self.x + self.y * self.y
    }

    /// Calculate the distance between `self` and `other`.
    pub fn distance(self, other: Vec2<T>) -> T
    where T: Float + Sub<Output = T> + Mul<Output = T> + Add<Output = T> {
        return (self - other).length()
    }

    /// Normalize `self` within a range of 0 to 1.
    pub fn normalize(self) -> Self
    where T: Float + Div<Output = T> {
        return self / self.length()
    }

    /// Normalize `self` within a range of `lower` and `upper`.
    pub fn normalize_between(self, lower: T, upper: T) -> Self
    where T: Float + Div<Output = T> {
        return self.normalize() * (upper - lower) + lower
    }

    /// Compute the dot product of `self` and `rhs`.
    pub fn dot(self, rhs: Vec2<T>) -> T
    where T: Mul<Output = T> + Add<Output = T> {
        return (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Linearly interpolate between `self` and `other` by `t`.
    pub fn lerp(self, other: Vec2<T>, t: T) -> Self
    where T: Float + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        return self * (T::one() - t) + other * t
    }

    /// Return the minimum of `self` and `other`.
    pub fn min(self, other: Vec2<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
        }
    }

    /// Return the maximum of `self` and `other`.
    pub fn max(self, other: Vec2<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
        }
    }

    /// Clamp `self` between `min` and `max`.
    pub fn clamp(self, min: Vec2<T>, max: Vec2<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x < min.x { min.x } else if self.x > max.x { max.x } else { self.x },
            y: if self.y < min.y { min.y } else if self.y > max.y { max.y } else { self.y },
        }
    }

    /// Returns a vector with the absolute value of each component.
    pub fn abs(self) -> Self
    where T: Float {
        return Self::new(self.x.abs(), self.y.abs())
    }

    /// Returns true if any component is NaN.
    pub fn is_nan(self) -> bool
    where T: Float {
        return self.x.is_nan() || self.y.is_nan()
    }

    /// Returns true if all components are finite.
    pub fn is_finite(self) -> bool
    where T: Float {
        return self.x.is_finite() && self.y.is_finite()
    }

    /// Returns true if any component is infinite.
    pub fn is_infinite(self) -> bool
    where T: Float {
        return self.x.is_infinite() || self.y.is_infinite()
    }

    /// Returns the sum of all components.
    pub fn sum(self) -> T
    where T: Add<Output = T> + Copy {
        return self.x + self.y
    }

    /// Returns the product of all components.
    pub fn product(self) -> T
    where T: Mul<Output = T> + Copy {
        return self.x * self.y
    }

    /// Returns the minimum element.
    pub fn min_element(self) -> T
    where T: PartialOrd + Copy {
        return if self.x < self.y { self.x } else { self.y }
    }

    /// Returns the maximum element.
    pub fn max_element(self) -> T
    where T: PartialOrd + Copy {
        return if self.x > self.y { self.x } else { self.y }
    }

    /// Returns true if any component is zero.
    pub fn any_zero(self) -> bool
    where T: PartialEq + Copy + num_traits::Zero {
        return self.x.is_zero() || self.y.is_zero()
    }

    /// Returns true if all components are zero.
    pub fn all_zero(self) -> bool
    where T: PartialEq + Copy + num_traits::Zero {
        return self.x.is_zero() && self.y.is_zero()
    }

    /// Reflects the vector about a normal.
    pub fn reflect(self, normal: Vec2<T>) -> Self
    where T: Float + Mul<Output = T> + Sub<Output = T> + Add<Output = T> {
        return self - normal * (T::from(2.0).unwrap() * self.dot(normal))
    }

    /// Projects this vector onto another.
    pub fn project_onto(self, other: Vec2<T>) -> Self
    where T: Float + Mul<Output = T> + Div<Output = T> + Add<Output = T> {
        let denom = other.length_squared();
        if denom.is_zero() {
            return Self::splat(T::zero())
        } else {
            return other * (self.dot(other) / denom)
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
impl<T: Copy + Sub<Output = T>> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

// T -> Vec2<T>
impl<T: Copy> From<T> for Vec2<T>{
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
