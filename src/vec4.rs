use core::fmt;
use super::{Vec2, Vec3};
use core::ops::{Index, IndexMut};
use core::ops::{Add, Sub, Mul, Div, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use num_traits::{Float, NumCast, Signed, One, Zero, float::FloatCore};

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

    /// A `Vec4<T>` with all components set to zero.
    pub fn zero() -> Self
    where T: Zero {
        return Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }

    /// A `Vec4<T>` with all components set to one.
    pub fn one() -> Self
    where T: One {
        return Self::new(T::one(), T::one(), T::one(), T::one())
    }

    /// A `Vec4<T>` with the x component set to one and all others to zero.
    pub fn unit_x() -> Self
    where T: Zero + One {
        return Self::new(T::one(), T::zero(), T::zero(), T::zero())
    }

    /// A `Vec4<T>` with the y component set to one and all others to zero.
    pub fn unit_y() -> Self
    where T: Zero + One {
        return Self::new(T::zero(), T::one(), T::zero(), T::zero())
    }

    /// A `Vec4<T>` with the z component set to one and all others to zero.
    pub fn unit_z() -> Self
    where T: Zero + One {
        return Self::new(T::zero(), T::zero(), T::one(), T::zero())
    }

    /// A `Vec4<T>` with the w component set to one and all others to zero.
    pub fn unit_w() -> Self
    where T: Zero + One {
        return Self::new(T::zero(), T::zero(), T::zero(), T::one())
    }

    /// Truncate a `Vec4<T>` into a `Vec3<T>` by dropping the w component.
    pub fn truncate(self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.z)
    }

    /// Return the length of the vector.
    pub fn length(self) -> T
    where T: Float {
        return self.length_squared().sqrt()
    }

    /// Return the squared length of the vector.
    pub fn length_squared(self) -> T
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Calculate the distance between `self` and `other`.
    pub fn distance(self, other: Vec4<T>) -> T
    where T: Float {
        return (self - other).length()
    }

    /// Calculate the squared distance between `self` and `other`.
    pub fn distance_squared(self, other: Vec4<T>) -> T
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

    /// Normalize the vector within a range of `lower` to `upper`.
    pub fn normalize_between(self, lower: T, upper: T) -> Self
    where T: Float {
        return self.normalize() * (upper - lower) + lower
    }

    /// Compute the dot product of `self` and `rhs`.
    pub fn dot(self, rhs: Vec4<T>) -> T
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise.
    pub fn mul_add(self, a: Vec4<T>, b: Vec4<T>) -> Self
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Self::new(
            self.x * a.x + b.x,
            self.y * a.y + b.y,
            self.z * a.z + b.z,
            self.w * a.w + b.w,
        )
    }

    /// Linearly interpolate between `self` and `other` by `t`.
    pub fn lerp(self, other: Vec4<T>, t: T) -> Self
    where T: Copy + One + Sub<Output = T> + Add<Output = T> + Mul<Output = T> {
        return self * (T::one() - t) + other * t
    }

    /// Return the component-wise minimum of `self` and `other`.
    pub fn min(self, other: Vec4<T>) -> Self
    where T: PartialOrd + Copy {
        return Self {
            x: if self.x < other.x { self.x } else { other.x },
            y: if self.y < other.y { self.y } else { other.y },
            z: if self.z < other.z { self.z } else { other.z },
            w: if self.w < other.w { self.w } else { other.w },
        }
    }

    /// Return the component-wise maximum of `self` and `other`.
    pub fn max(self, other: Vec4<T>) -> Self
    where T: PartialOrd + Copy {
        return Self {
            x: if self.x > other.x { self.x } else { other.x },
            y: if self.y > other.y { self.y } else { other.y },
            z: if self.z > other.z { self.z } else { other.z },
            w: if self.w > other.w { self.w } else { other.w },
        }
    }

    /// Clamp `self` between `min` and `max` component-wise.
    pub fn clamp(self, min: Vec4<T>, max: Vec4<T>) -> Self
    where T: PartialOrd + Copy {
        return self.max(min).min(max)
    }

    /// Returns a vector with the absolute value of each component.
    pub fn abs(self) -> Self
    where T: Signed {
        return Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    /// Returns a vector with the sign of each component.
    pub fn signum(self) -> Self
    where T: Signed {
        return Self::new(self.x.signum(), self.y.signum(), self.z.signum(), self.w.signum())
    }

    /// Returns a vector with the reciprocal of each component.
    pub fn recip(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.recip(), self.y.recip(), self.z.recip(), self.w.recip())
    }

    /// Returns a vector with the floor of each component.
    pub fn floor(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.floor(), self.y.floor(), self.z.floor(), self.w.floor())
    }

    /// Returns a vector with the ceil of each component.
    pub fn ceil(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
    }

    /// Returns a vector with the round of each component.
    pub fn round(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.round(), self.y.round(), self.z.round(), self.w.round())
    }

    /// Returns a vector with the fractional part of each component.
    pub fn fract(self) -> Self
    where T: FloatCore {
        return Self::new(self.x.fract(), self.y.fract(), self.z.fract(), self.w.fract())
    }

    /// Returns a vector with each component raised to the given power.
    pub fn powf(self, n: T) -> Self
    where T: Float {
        return Self::new(self.x.powf(n), self.y.powf(n), self.z.powf(n), self.w.powf(n))
    }

    /// Returns true if any component is NaN.
    pub fn is_nan(self) -> bool
    where T: FloatCore {
        return self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }

    /// Returns true if all components are finite.
    pub fn is_finite(self) -> bool
    where T: FloatCore {
        return self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Returns true if any component is infinite.
    pub fn is_infinite(self) -> bool
    where T: FloatCore {
        return self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite() || self.w.is_infinite()
    }

    /// Returns the sum of all components.
    pub fn sum(self) -> T
    where T: Add<Output = T> {
        return self.x + self.y + self.z + self.w
    }

    /// Returns the product of all components.
    pub fn product(self) -> T
    where T: Mul<Output = T> {
        return self.x * self.y * self.z * self.w
    }

    /// Returns the minimum element.
    pub fn min_element(self) -> T
    where T: PartialOrd + Copy {
        let mut min = self.x;
        if self.y < min { min = self.y; }
        if self.z < min { min = self.z; }
        if self.w < min { min = self.w; }
        return min
    }

    /// Returns the maximum element.
    pub fn max_element(self) -> T
    where T: PartialOrd + Copy {
        let mut max = self.x;
        if self.y > max { max = self.y; }
        if self.z > max { max = self.z; }
        if self.w > max { max = self.w; }
        return max
    }

    /// Returns true if any component is zero.
    pub fn any_zero(self) -> bool
    where T: PartialEq + Zero {
        return self.x.is_zero() || self.y.is_zero() || self.z.is_zero() || self.w.is_zero()
    }

    /// Returns true if all components are zero.
    pub fn all_zero(self) -> bool
    where T: PartialEq + Zero {
        return self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }

    /// Reflects the vector about a normal.
    pub fn reflect(self, normal: Vec4<T>) -> Self
    where T: Copy + NumCast + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        let two: T = NumCast::from(2.0f64).unwrap();
        return self - normal * (two * self.dot(normal))
    }

    /// Projects this vector onto another.
    pub fn project_onto(self, other: Vec4<T>) -> Self
    where T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Zero + PartialEq {
        let denom = other.length_squared();
        if denom.is_zero() {
            return Self::zero()
        }
        return other * (self.dot(other) / denom)
    }

    /// Rejects this vector from another (component perpendicular to `other`).
    pub fn reject_from(self, other: Vec4<T>) -> Self
    where T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Zero + PartialEq {
        return self - self.project_onto(other)
    }

    /// Converts this vector to an array.
    pub fn to_array(self) -> [T; 4] {
        return [self.x, self.y, self.z, self.w]
    }
}

impl<T: fmt::Display> fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => return &self.x,
            1 => return &self.y,
            2 => return &self.z,
            3 => return &self.w,
            _ => panic!("index out of bounds: Vec4 has 4 components but index is {}", index),
        }
    }
}

impl<T> IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => return &mut self.x,
            1 => return &mut self.y,
            2 => return &mut self.z,
            3 => return &mut self.w,
            _ => panic!("index out of bounds: Vec4 has 4 components but index is {}", index),
        }
    }
}

// Vec4<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
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
        return Vec4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
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
        return Vec4::new(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs)
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
        return Vec4::new(self.x - rhs, self.y - rhs, self.z - rhs, self.w - rhs)
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
        return Vec4::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
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
        return Vec4::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
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
        return Vec4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
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
        return Vec4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
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

// -Vec4<T>
impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Vec4<T>;

    fn neg(self) -> Self::Output {
        return Vec4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

// T -> Vec4<T>
impl<T: Copy> From<T> for Vec4<T> {
    fn from(v: T) -> Self {
        return Self::new(v, v, v, v)
    }
}

// [T; 4] -> Vec4<T>
impl<T> From<[T; 4]> for Vec4<T> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        return Self::new(x, y, z, w)
    }
}

// [Vec2<T>; 2] -> Vec4<T>
impl<T> From<[Vec2<T>; 2]> for Vec4<T> {
    fn from([v1, v2]: [Vec2<T>; 2]) -> Self {
        return Self::new(v1.x, v1.y, v2.x, v2.y)
    }
}

// (T, T, T, T) -> Vec4<T>
impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        return Self::new(x, y, z, w)
    }
}

// (Vec3<T>, T) -> Vec4<T>
impl<T> From<(Vec3<T>, T)> for Vec4<T> {
    fn from((v, t): (Vec3<T>, T)) -> Self {
        return Self::new(v.x, v.y, v.z, t)
    }
}

// (T, Vec3<T>) -> Vec4<T>
impl<T> From<(T, Vec3<T>)> for Vec4<T> {
    fn from((t, v): (T, Vec3<T>)) -> Self {
        return Self::new(t, v.x, v.y, v.z)
    }
}

// (Vec2<T>, T, T) -> Vec4<T>
impl<T> From<(Vec2<T>, T, T)> for Vec4<T> {
    fn from((v, z, w): (Vec2<T>, T, T)) -> Self {
        return Self::new(v.x, v.y, z, w)
    }
}

// (T, T, Vec2<T>) -> Vec4<T>
impl<T> From<(T, T, Vec2<T>)> for Vec4<T> {
    fn from((t1, t2, v): (T, T, Vec2<T>)) -> Self {
        return Self::new(t1, t2, v.x, v.y)
    }
}

// (T, Vec2<T>, T) -> Vec4<T>
impl<T> From<(T, Vec2<T>, T)> for Vec4<T> {
    fn from((t1, v, t2): (T, Vec2<T>, T)) -> Self {
        return Self::new(t1, v.x, v.y, t2)
    }
}

// (Vec2<T>, Vec2<T>) -> Vec4<T>
impl<T> From<(Vec2<T>, Vec2<T>)> for Vec4<T> {
    fn from((v1, v2): (Vec2<T>, Vec2<T>)) -> Self {
        return Self::new(v1.x, v1.y, v2.x, v2.y)
    }
}

// Vec4<T> -> [T; 4]
impl<T> From<Vec4<T>> for [T; 4] {
    fn from(v: Vec4<T>) -> Self {
        return [v.x, v.y, v.z, v.w]
    }
}

// Vec4<T> -> [Vec2<T>; 2]
impl<T> From<Vec4<T>> for [Vec2<T>; 2] {
    fn from(v: Vec4<T>) -> Self {
        return [Vec2::new(v.x, v.y), Vec2::new(v.z, v.w)]
    }
}

// Vec4<T> -> (T, T, T, T)
impl<T> From<Vec4<T>> for (T, T, T, T) {
    fn from(v: Vec4<T>) -> Self {
        return (v.x, v.y, v.z, v.w)
    }
}

// Vec4<T> -> (Vec2<T>, Vec2<T>)
impl<T> From<Vec4<T>> for (Vec2<T>, Vec2<T>) {
    fn from(v: Vec4<T>) -> Self {
        return (Vec2::new(v.x, v.y), Vec2::new(v.z, v.w))
    }
}

// Vec4<T> -> (T, T, Vec2<T>)
impl<T> From<Vec4<T>> for (T, T, Vec2<T>) {
    fn from(v: Vec4<T>) -> Self {
        return (v.x, v.y, Vec2::new(v.z, v.w))
    }
}

// Vec4<T> -> (Vec2<T>, T, T)
impl<T> From<Vec4<T>> for (Vec2<T>, T, T) {
    fn from(v: Vec4<T>) -> Self {
        return (Vec2::new(v.x, v.y), v.z, v.w)
    }
}

// Vec4<T> -> (T, Vec2<T>, T)
impl<T> From<Vec4<T>> for (T, Vec2<T>, T) {
    fn from(v: Vec4<T>) -> Self {
        return (v.x, Vec2::new(v.y, v.z), v.w)
    }
}

// Vec4<T> -> (Vec3<T>, T)
impl<T> From<Vec4<T>> for (Vec3<T>, T) {
    fn from(v: Vec4<T>) -> Self {
        return (Vec3::new(v.x, v.y, v.z), v.w)
    }
}

// Vec4<T> -> (T, Vec3<T>)
impl<T> From<Vec4<T>> for (T, Vec3<T>) {
    fn from(v: Vec4<T>) -> Self {
        return (v.x, Vec3::new(v.y, v.z, v.w))
    }
}
