use super::Vec2;
use core::fmt;
use crate::units::Rad;
use core::ops::{Index, IndexMut};
use core::ops::{Add, Sub, Mul, Div, Neg};
use num_traits::{Float, One, Zero, float::FloatCore};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

/// A 2x2 column-major matrix of type T.
///
/// Columns are stored as `Vec2<T>` where `x` is column 0 and `y` is column 1.
/// This means the matrix layout in memory is:
/// ```text
///   col0   col1
///  [m00]  [m01]
///  [m10]  [m11]
/// ```
/// where `x = Vec2(m00, m10)` and `y = Vec2(m01, m11)`.
#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

impl<T> Mat2<T> {
    /// Create a new `Mat2<T>` from two column vectors.
    pub const fn new(x: Vec2<T>, y: Vec2<T>) -> Self {
        return Self { x, y }
    }

    /// Create a `Mat2<T>` from individual elements (column-major order).
    pub const fn from_cols(m00: T, m10: T, m01: T, m11: T) -> Self {
        return Self {
            x: Vec2::new(m00, m10),
            y: Vec2::new(m01, m11),
        }
    }

    /// Returns the identity matrix.
    pub fn identity() -> Self
    where T: Zero + One {
        return Self::new(
            Vec2::new(T::one(), T::zero()),
            Vec2::new(T::zero(), T::one()),
        )
    }

    /// Returns the zero matrix.
    pub fn zero() -> Self
    where T: Zero {
        return Self::new(Vec2::zero(), Vec2::zero())
    }

    /// Returns the determinant of the matrix.
    pub fn determinant(self) -> T
    where T: Copy + Mul<Output = T> + Sub<Output = T> {
        return self.x.x * self.y.y - self.y.x * self.x.y
    }

    /// Returns the transpose of the matrix.
    pub fn transpose(self) -> Self {
        return Self::new(
            Vec2::new(self.x.x, self.y.x),
            Vec2::new(self.x.y, self.y.y),
        )
    }

    /// Returns the inverse of the matrix, or `None` if the determinant is zero.
    pub fn inverse(self) -> Option<Self>
    where T: FloatCore {
        let det = self.determinant();
        if det.is_zero() {
            return None
        }
        let inv_det = det.recip();
        return Some(Self::new(
            Vec2::new(self.y.y * inv_det, -self.x.y * inv_det),
            Vec2::new(-self.y.x * inv_det, self.x.x * inv_det),
        ))
    }

    /// Creates a 2x2 rotation matrix from an angle.
    pub fn from_angle(angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        return Self::new(
            Vec2::new(c, s),
            Vec2::new(-s, c),
        )
    }

    /// Creates a 2x2 scale matrix.
    pub fn from_scale(scale: Vec2<T>) -> Self
    where T: Zero {
        return Self::new(
            Vec2::new(scale.x, T::zero()),
            Vec2::new(T::zero(), scale.y),
        )
    }

    /// Creates a uniform scale matrix.
    pub fn from_scale_uniform(scale: T) -> Self
    where T: Copy + Zero {
        return Self::new(
            Vec2::new(scale, T::zero()),
            Vec2::new(T::zero(), scale),
        )
    }

    /// Creates a scale + rotation matrix.
    pub fn from_scale_angle(scale: Vec2<T>, angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        return Self::new(
            Vec2::new(scale.x * c, scale.x * s),
            Vec2::new(-scale.y * s, scale.y * c),
        )
    }

    /// Multiplies this matrix by a column vector, returning the resulting vector.
    pub fn mul_vec2(self, rhs: Vec2<T>) -> Vec2<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Vec2::new(
            self.x.x * rhs.x + self.y.x * rhs.y,
            self.x.y * rhs.x + self.y.y * rhs.y,
        )
    }

    /// Multiplies this matrix by another matrix.
    pub fn mul_mat2(self, rhs: Mat2<T>) -> Mat2<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Mat2::new(
            self.mul_vec2(rhs.x),
            self.mul_vec2(rhs.y),
        )
    }

    /// Returns the matrix as a flat array in column-major order.
    pub fn to_cols_array(self) -> [T; 4] {
        return [self.x.x, self.x.y, self.y.x, self.y.y]
    }

    /// Creates a matrix from a flat array in column-major order.
    pub fn from_cols_array(arr: [T; 4]) -> Self
    where T: Copy {
        return Self::new(
            Vec2::new(arr[0], arr[1]),
            Vec2::new(arr[2], arr[3]),
        )
    }

    /// Returns a reference to a column by index.
    pub fn col(&self, index: usize) -> &Vec2<T> {
        match index {
            0 => return &self.x,
            1 => return &self.y,
            _ => panic!("index out of bounds: Mat2 has 2 columns but index is {}", index),
        }
    }

    /// Returns a mutable reference to a column by index.
    pub fn col_mut(&mut self, index: usize) -> &mut Vec2<T> {
        match index {
            0 => return &mut self.x,
            1 => return &mut self.y,
            _ => panic!("index out of bounds: Mat2 has 2 columns but index is {}", index),
        }
    }

    /// Returns a row by index.
    pub fn row(&self, index: usize) -> Vec2<T>
    where T: Copy {
        match index {
            0 => return Vec2::new(self.x.x, self.y.x),
            1 => return Vec2::new(self.x.y, self.y.y),
            _ => panic!("index out of bounds: Mat2 has 2 rows but index is {}", index),
        }
    }

    /// Returns true if any element is NaN.
    pub fn is_nan(self) -> bool
    where T: FloatCore {
        return self.x.is_nan() || self.y.is_nan()
    }

    /// Returns true if all elements are finite.
    pub fn is_finite(self) -> bool
    where T: FloatCore {
        return self.x.is_finite() && self.y.is_finite()
    }
}

impl<T: fmt::Display> fmt::Display for Mat2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Mat2([{}, {}], [{}, {}])",
            self.x.x, self.y.x,
            self.x.y, self.y.y)
    }
}

impl<T> Index<usize> for Mat2<T> {
    type Output = Vec2<T>;

    fn index(&self, index: usize) -> &Self::Output {
        return self.col(index)
    }
}

impl<T> IndexMut<usize> for Mat2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.col_mut(index)
    }
}

// Mat2 * Mat2
impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul(self, rhs: Mat2<T>) -> Self::Output {
        return self.mul_mat2(rhs)
    }
}

// Mat2 *= Mat2
impl<T: Copy + Mul<Output = T> + Add<Output = T>> MulAssign<Mat2<T>> for Mat2<T> {
    fn mul_assign(&mut self, rhs: Mat2<T>) {
        *self = self.mul_mat2(rhs);
    }
}

// Mat2 * Vec2
impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Vec2<T>> for Mat2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        return self.mul_vec2(rhs)
    }
}

// Mat2 * scalar
impl<T: Copy + Mul<Output = T>> Mul<T> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Mat2::new(self.x * rhs, self.y * rhs)
    }
}

// Mat2 *= scalar
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Mat2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

// Mat2 / scalar
impl<T: Copy + Div<Output = T>> Div<T> for Mat2<T> {
    type Output = Mat2<T>;

    fn div(self, rhs: T) -> Self::Output {
        return Mat2::new(self.x / rhs, self.y / rhs)
    }
}

// Mat2 /= scalar
impl<T: Copy + Div<Output = T>> DivAssign<T> for Mat2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

// Mat2 + Mat2
impl<T: Copy + Add<Output = T>> Add<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn add(self, rhs: Mat2<T>) -> Self::Output {
        return Mat2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// Mat2 += Mat2
impl<T: Copy + Add<Output = T>> AddAssign<Mat2<T>> for Mat2<T> {
    fn add_assign(&mut self, rhs: Mat2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

// Mat2 - Mat2
impl<T: Copy + Sub<Output = T>> Sub<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn sub(self, rhs: Mat2<T>) -> Self::Output {
        return Mat2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// Mat2 -= Mat2
impl<T: Copy + Sub<Output = T>> SubAssign<Mat2<T>> for Mat2<T> {
    fn sub_assign(&mut self, rhs: Mat2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

// -Mat2
impl<T: Neg<Output = T>> Neg for Mat2<T> {
    type Output = Mat2<T>;

    fn neg(self) -> Self::Output {
        return Mat2::new(-self.x, -self.y)
    }
}

// [T; 4] -> Mat2<T>
impl<T> From<[T; 4]> for Mat2<T> {
    fn from(arr: [T; 4]) -> Self {
        let [m00, m10, m01, m11] = arr;
        return Self::new(Vec2::new(m00, m10), Vec2::new(m01, m11))
    }
}

// Mat2<T> -> [T; 4]
impl<T> From<Mat2<T>> for [T; 4] {
    fn from(m: Mat2<T>) -> Self {
        return [m.x.x, m.x.y, m.y.x, m.y.y]
    }
}

// [Vec2<T>; 2] -> Mat2<T>
impl<T> From<[Vec2<T>; 2]> for Mat2<T> {
    fn from([x, y]: [Vec2<T>; 2]) -> Self {
        return Self::new(x, y)
    }
}

// Mat2<T> -> [Vec2<T>; 2]
impl<T> From<Mat2<T>> for [Vec2<T>; 2] {
    fn from(m: Mat2<T>) -> Self {
        return [m.x, m.y]
    }
}
