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
        return self.length_squared().sqrt()
    }

    /// Return the squared length of the vector.
    pub fn length_squared(self) -> T
    where T: Float + Mul<Output = T> + Add<Output = T> {
        return self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculate the distance between `self` and `other`.
    pub fn distance(self, other: Vec3<T>) -> T
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

    /// Linearly interpolate between `self` and `other` by `t`.
    pub fn lerp(self, other: Vec3<T>, t: T) -> Self
    where T: Float + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        return self * (T::one() - t) + other * t
    }

    /// Return the minimum of `self` and `other`.
    pub fn min(self, other: Vec3<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
            z: if self.z < other.z { self.z } else { other.z },
        }
    }

    /// Return the maximum of `self` and `other`.
    pub fn max(self, other: Vec3<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
            z: if self.z > other.z { self.z } else { other.z },
        }
    }

    /// Clamp `self` between `min` and `max`.
    pub fn clamp(self, min: Vec3<T>, max: Vec3<T>) -> Self
    where T: Float + PartialOrd {
        return Self {
            x: if self.x < min.x { min.x } else if self.x > max.x { max.x } else { self.x },
            y: if self.y < min.y { min.y } else if self.y > max.y { max.y } else { self.y },
            z: if self.z < min.z { min.z } else if self.z > max.z { max.z } else { self.z },
        }
    }

    /// Returns a vector with the absolute value of each component.
    pub fn abs(self) -> Self
    where T: Float {
        return Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Returns true if any component is NaN.
    pub fn is_nan(self) -> bool
    where T: Float {
        return self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    /// Returns true if all components are finite.
    pub fn is_finite(self) -> bool
    where T: Float {
        return self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Returns true if any component is infinite.
    pub fn is_infinite(self) -> bool
    where T: Float {
        return self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite()
    }

    /// Returns the sum of all components.
    pub fn sum(self) -> T
    where T: Add<Output = T> + Copy {
        return self.x + self.y + self.z
    }

    /// Returns the product of all components.
    pub fn product(self) -> T
    where T: Mul<Output = T> + Copy {
        return self.x * self.y * self.z
    }

    /// Returns the minimum element.
    pub fn min_element(self) -> T
    where T: PartialOrd + Copy {
        let mut min = self.x;
        if self.y < min { min = self.y; }
        if self.z < min { min = self.z; }
        return min
    }

    /// Returns the maximum element.
    pub fn max_element(self) -> T
    where T: PartialOrd + Copy {
        let mut max = self.x;
        if self.y > max { max = self.y; }
        if self.z > max { max = self.z; }
        return max
    }

    /// Returns true if any component is zero.
    pub fn any_zero(self) -> bool
    where T: PartialEq + Copy + num_traits::Zero {
        return self.x.is_zero() || self.y.is_zero() || self.z.is_zero()
    }

    /// Returns true if all components are zero.
    pub fn all_zero(self) -> bool
    where T: PartialEq + Copy + num_traits::Zero {
        return self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }

    /// Reflects the vector about a normal.
    pub fn reflect(self, normal: Vec3<T>) -> Self
    where T: Float + Mul<Output = T> + Sub<Output = T> + Add<Output = T> {
        return self - normal * (T::from(2.0).unwrap() * self.dot(normal))
    }

    /// Projects this vector onto another.
    pub fn project_onto(self, other: Vec3<T>) -> Self
    where T: Float + Mul<Output = T> + Div<Output = T> + Add<Output = T> {
        let denom = other.length_squared();
        if denom.is_zero() {
            return Self::splat(T::zero())
        } else {
            return other * (self.dot(other) / denom)
        }
    }
}

// Vec3<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
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
        return Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
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
        return Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
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
        return Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
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
        return Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
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
        return Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
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
        return Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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
        return Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
        return Self::new(v, v, v)
    }
}

// [T; 3] -> Vec3<T>
impl<T> From<[T; 3]> for Vec3<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        return Self::new(x, y, z)
    }
}

// (T, T, T) -> Vec3<T>
impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        return Self::new(x, y, z)
    }
}

// (T, Vec2<T>) -> Vec3<T>
impl<T> From<(T, Vec2<T>)> for Vec3<T> {
    fn from((t, v): (T, Vec2<T>)) -> Self {
        return Self::new(t, v.x, v.y)
    }
}

// (Vec2<T>, T) -> Vec3<T>
impl<T> From<(Vec2<T>, T)> for Vec3<T> {
    fn from((v, t): (Vec2<T>, T)) -> Self {
        return Self::new(v.x, v.y, t)
    }
}

// Vec3<T> -> [T; 3]
impl<T> From<Vec3<T>> for [T; 3] {
    fn from(v: Vec3<T>) -> Self {
        return [v.x, v.y, v.z]
    }
}

// Vec3<T> -> (T, T, T)
impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(v: Vec3<T>) -> Self {
        return (v.x, v.y, v.z)
    }
}

// Vec3<T> -> (T, Vec2<T>)
impl<T> From<Vec3<T>> for (T, Vec2<T>) {
    fn from(v: Vec3<T>) -> Self {
        return (v.x, Vec2::new(v.y, v.z))
    }
}

// Vec3<T> -> (Vec2<T>, T)
impl<T> From<Vec3<T>> for (Vec2<T>, T) {
    fn from(v: Vec3<T>) -> Self {
        return (Vec2::new(v.x, v.y), v.z)
    }
}
