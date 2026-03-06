use core::fmt;
use crate::units::Rad;
use super::{Vec2, Vec3, Mat2};
use core::ops::{Index, IndexMut};
use core::ops::{Add, Sub, Mul, Div, Neg};
use num_traits::{Float, One, Zero, float::FloatCore};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

/// A 3x3 column-major matrix of type T.
///
/// Columns are stored as `Vec3<T>` where `x` is column 0, `y` is column 1,
/// and `z` is column 2.
#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

impl<T> Mat3<T> {
    /// Create a new `Mat3<T>` from three column vectors.
    pub const fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        return Self { x, y, z }
    }

    /// Create a `Mat3<T>` from individual elements (column-major order).
    #[allow(clippy::too_many_arguments)]
    pub const fn from_cols(
        m00: T, m10: T, m20: T,
        m01: T, m11: T, m21: T,
        m02: T, m12: T, m22: T,
    ) -> Self {
        return Self {
            x: Vec3::new(m00, m10, m20),
            y: Vec3::new(m01, m11, m21),
            z: Vec3::new(m02, m12, m22),
        }
    }

    /// Returns the identity matrix.
    pub fn identity() -> Self
    where T: Zero + One {
        return Self::new(
            Vec3::new(T::one(), T::zero(), T::zero()),
            Vec3::new(T::zero(), T::one(), T::zero()),
            Vec3::new(T::zero(), T::zero(), T::one()),
        )
    }

    /// Returns the zero matrix.
    pub fn zero() -> Self
    where T: Zero {
        return Self::new(Vec3::zero(), Vec3::zero(), Vec3::zero())
    }

    /// Returns the determinant of the matrix.
    pub fn determinant(self) -> T
    where T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T> {
        let a = self.x.x;
        let b = self.y.x;
        let c = self.z.x;
        let d = self.x.y;
        let e = self.y.y;
        let f = self.z.y;
        let g = self.x.z;
        let h = self.y.z;
        let i = self.z.z;
        return a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    /// Returns the transpose of the matrix.
    pub fn transpose(self) -> Self {
        return Self::new(
            Vec3::new(self.x.x, self.y.x, self.z.x),
            Vec3::new(self.x.y, self.y.y, self.z.y),
            Vec3::new(self.x.z, self.y.z, self.z.z),
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

        let a = self.x.x;
        let b = self.y.x;
        let c = self.z.x;
        let d = self.x.y;
        let e = self.y.y;
        let f = self.z.y;
        let g = self.x.z;
        let h = self.y.z;
        let i = self.z.z;

        // Cofactor matrix transposed * 1/det
        return Some(Self::new(
            Vec3::new(
                (e * i - f * h) * inv_det,
                (f * g - d * i) * inv_det,
                (d * h - e * g) * inv_det,
            ),
            Vec3::new(
                (c * h - b * i) * inv_det,
                (a * i - c * g) * inv_det,
                (b * g - a * h) * inv_det,
            ),
            Vec3::new(
                (b * f - c * e) * inv_det,
                (c * d - a * f) * inv_det,
                (a * e - b * d) * inv_det,
            ),
        ))
    }

    /// Creates a 3x3 translation matrix for 2D transformations.
    pub fn from_translation(translation: Vec2<T>) -> Self
    where T: Zero + One {
        return Self::new(
            Vec3::new(T::one(), T::zero(), T::zero()),
            Vec3::new(T::zero(), T::one(), T::zero()),
            Vec3::new(translation.x, translation.y, T::one()),
        )
    }

    /// Creates a 3x3 rotation matrix from an angle (rotation about the z axis).
    pub fn from_angle(angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        return Self::new(
            Vec3::new(c, s, T::zero()),
            Vec3::new(-s, c, T::zero()),
            Vec3::new(T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a 3x3 non-uniform scale matrix.
    pub fn from_scale(scale: Vec2<T>) -> Self
    where T: Zero + One {
        return Self::new(
            Vec3::new(scale.x, T::zero(), T::zero()),
            Vec3::new(T::zero(), scale.y, T::zero()),
            Vec3::new(T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a 3x3 matrix from a 2x2 matrix, extending with an identity z column/row.
    pub fn from_mat2(m: Mat2<T>) -> Self
    where T: Zero + One {
        return Self::new(
            Vec3::new(m.x.x, m.x.y, T::zero()),
            Vec3::new(m.y.x, m.y.y, T::zero()),
            Vec3::new(T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a right-handed view matrix using a facing direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    pub fn look_to_rh(dir: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        let f = dir;
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        return Self::new(
            Vec3::new(s.x, u.x, -f.x),
            Vec3::new(s.y, u.y, -f.y),
            Vec3::new(s.z, u.z, -f.z),
        )
    }

    /// Creates a left-handed view matrix using a facing direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    pub fn look_to_lh(dir: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_rh(-dir, up)
    }

    /// Creates a right-handed view matrix using a camera position, a focal point and
    /// an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    pub fn look_at_rh(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_rh((center - eye).normalize(), up)
    }

    /// Creates a left-handed view matrix using a camera position, a focal point and
    /// an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    pub fn look_at_lh(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_lh((center - eye).normalize(), up)
    }

    /// Multiplies this matrix by a column vector, returning the resulting vector.
    pub fn mul_vec3(self, rhs: Vec3<T>) -> Vec3<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Vec3::new(
            self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z,
            self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z,
            self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z,
        )
    }

    /// Multiplies this matrix by another matrix.
    pub fn mul_mat3(self, rhs: Mat3<T>) -> Mat3<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Mat3::new(
            self.mul_vec3(rhs.x),
            self.mul_vec3(rhs.y),
            self.mul_vec3(rhs.z),
        )
    }

    /// Returns the matrix as a flat array in column-major order.
    pub fn to_cols_array(self) -> [T; 9] {
        return [
            self.x.x, self.x.y, self.x.z,
            self.y.x, self.y.y, self.y.z,
            self.z.x, self.z.y, self.z.z,
        ]
    }

    /// Creates a matrix from a flat array in column-major order.
    pub fn from_cols_array(arr: [T; 9]) -> Self
    where T: Copy {
        return Self::new(
            Vec3::new(arr[0], arr[1], arr[2]),
            Vec3::new(arr[3], arr[4], arr[5]),
            Vec3::new(arr[6], arr[7], arr[8]),
        )
    }

    /// Returns a reference to a column by index.
    pub fn col(&self, index: usize) -> &Vec3<T> {
        match index {
            0 => return &self.x,
            1 => return &self.y,
            2 => return &self.z,
            _ => panic!("index out of bounds: Mat3 has 3 columns but index is {}", index),
        }
    }

    /// Returns a mutable reference to a column by index.
    pub fn col_mut(&mut self, index: usize) -> &mut Vec3<T> {
        match index {
            0 => return &mut self.x,
            1 => return &mut self.y,
            2 => return &mut self.z,
            _ => panic!("index out of bounds: Mat3 has 3 columns but index is {}", index),
        }
    }

    /// Returns a row by index.
    pub fn row(&self, index: usize) -> Vec3<T>
    where T: Copy {
        match index {
            0 => return Vec3::new(self.x.x, self.y.x, self.z.x),
            1 => return Vec3::new(self.x.y, self.y.y, self.z.y),
            2 => return Vec3::new(self.x.z, self.y.z, self.z.z),
            _ => panic!("index out of bounds: Mat3 has 3 rows but index is {}", index),
        }
    }

    /// Returns true if any element is NaN.
    pub fn is_nan(self) -> bool
    where T: FloatCore {
        return self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    /// Returns true if all elements are finite.
    pub fn is_finite(self) -> bool
    where T: FloatCore {
        return self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl<T: fmt::Display + Copy> fmt::Display for Mat3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Mat3([{}, {}, {}], [{}, {}, {}], [{}, {}, {}])",
            self.x.x, self.y.x, self.z.x,
            self.x.y, self.y.y, self.z.y,
            self.x.z, self.y.z, self.z.z)
    }
}

impl<T> Index<usize> for Mat3<T> {
    type Output = Vec3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        return self.col(index)
    }
}

impl<T> IndexMut<usize> for Mat3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.col_mut(index)
    }
}

// Mat3 * Mat3
impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul(self, rhs: Mat3<T>) -> Self::Output {
        return self.mul_mat3(rhs)
    }
}

// Mat3 *= Mat3
impl<T: Copy + Mul<Output = T> + Add<Output = T>> MulAssign<Mat3<T>> for Mat3<T> {
    fn mul_assign(&mut self, rhs: Mat3<T>) {
        *self = self.mul_mat3(rhs);
    }
}

// Mat3 * Vec3
impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Vec3<T>> for Mat3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        return self.mul_vec3(rhs)
    }
}

// Mat3 * scalar
impl<T: Copy + Mul<Output = T>> Mul<T> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Mat3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Mat3 *= scalar
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Mat3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

// Mat3 / scalar
impl<T: Copy + Div<Output = T>> Div<T> for Mat3<T> {
    type Output = Mat3<T>;

    fn div(self, rhs: T) -> Self::Output {
        return Mat3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// Mat3 /= scalar
impl<T: Copy + Div<Output = T>> DivAssign<T> for Mat3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

// Mat3 + Mat3
impl<T: Copy + Add<Output = T>> Add<Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn add(self, rhs: Mat3<T>) -> Self::Output {
        return Mat3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

// Mat3 += Mat3
impl<T: Copy + Add<Output = T>> AddAssign<Mat3<T>> for Mat3<T> {
    fn add_assign(&mut self, rhs: Mat3<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

// Mat3 - Mat3
impl<T: Copy + Sub<Output = T>> Sub<Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn sub(self, rhs: Mat3<T>) -> Self::Output {
        return Mat3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

// Mat3 -= Mat3
impl<T: Copy + Sub<Output = T>> SubAssign<Mat3<T>> for Mat3<T> {
    fn sub_assign(&mut self, rhs: Mat3<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

// -Mat3
impl<T: Neg<Output = T>> Neg for Mat3<T> {
    type Output = Mat3<T>;

    fn neg(self) -> Self::Output {
        return Mat3::new(-self.x, -self.y, -self.z)
    }
}

// [Vec3<T>; 3] -> Mat3<T>
impl<T> From<[Vec3<T>; 3]> for Mat3<T> {
    fn from([x, y, z]: [Vec3<T>; 3]) -> Self {
        return Self::new(x, y, z)
    }
}

// Mat3<T> -> [Vec3<T>; 3]
impl<T> From<Mat3<T>> for [Vec3<T>; 3] {
    fn from(m: Mat3<T>) -> Self {
        return [m.x, m.y, m.z]
    }
}

// Mat2<T> -> Mat3<T>
impl<T: Zero + One> From<Mat2<T>> for Mat3<T> {
    fn from(m: Mat2<T>) -> Self {
        return Self::from_mat2(m)
    }
}
