use core::fmt;
use crate::units::Rad;
use super::{Vec3, Vec4, Mat3};
use core::ops::{Index, IndexMut};
use core::ops::{Add, Sub, Mul, Div, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use num_traits::{Float, NumCast, One, Zero, float::FloatCore};

/// A 4x4 column-major matrix of type T.
///
/// Columns are stored as `Vec4<T>` where `x` is column 0, `y` is column 1,
/// `z` is column 2, and `w_axis` is column 3.
#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w_axis: Vec4<T>,
}

impl<T> Mat4<T> {
    /// Create a new `Mat4<T>` from four column vectors.
    pub const fn new(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w_axis: Vec4<T>) -> Self {
        return Self { x, y, z, w_axis }
    }

    /// Create a `Mat4<T>` from individual elements (column-major order).
    #[allow(clippy::too_many_arguments)]
    pub const fn from_cols(
        m00: T, m10: T, m20: T, m30: T,
        m01: T, m11: T, m21: T, m31: T,
        m02: T, m12: T, m22: T, m32: T,
        m03: T, m13: T, m23: T, m33: T,
    ) -> Self {
        return Self {
            x: Vec4::new(m00, m10, m20, m30),
            y: Vec4::new(m01, m11, m21, m31),
            z: Vec4::new(m02, m12, m22, m32),
            w_axis: Vec4::new(m03, m13, m23, m33),
        }
    }

    /// Returns the identity matrix.
    pub fn identity() -> Self
    where T: Zero + One {
        return Self::new(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Returns the zero matrix.
    pub fn zero() -> Self
    where T: Zero {
        return Self::new(Vec4::zero(), Vec4::zero(), Vec4::zero(), Vec4::zero())
    }

    /// Returns the determinant of the matrix.
    pub fn determinant(self) -> T
    where T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T> {
        // Laplace expansion along the first row
        let a = self.x.x;
        let b = self.y.x;
        let c = self.z.x;
        let d = self.w_axis.x;

        // Minors of the first row
        let minor_a = Mat3::new(
            Vec3::new(self.y.y, self.y.z, self.y.w),
            Vec3::new(self.z.y, self.z.z, self.z.w),
            Vec3::new(self.w_axis.y, self.w_axis.z, self.w_axis.w),
        ).determinant();

        let minor_b = Mat3::new(
            Vec3::new(self.x.y, self.x.z, self.x.w),
            Vec3::new(self.z.y, self.z.z, self.z.w),
            Vec3::new(self.w_axis.y, self.w_axis.z, self.w_axis.w),
        ).determinant();

        let minor_c = Mat3::new(
            Vec3::new(self.x.y, self.x.z, self.x.w),
            Vec3::new(self.y.y, self.y.z, self.y.w),
            Vec3::new(self.w_axis.y, self.w_axis.z, self.w_axis.w),
        ).determinant();

        let minor_d = Mat3::new(
            Vec3::new(self.x.y, self.x.z, self.x.w),
            Vec3::new(self.y.y, self.y.z, self.y.w),
            Vec3::new(self.z.y, self.z.z, self.z.w),
        ).determinant();

        return a * minor_a - b * minor_b + c * minor_c - d * minor_d
    }

    /// Returns the transpose of the matrix.
    pub fn transpose(self) -> Self {
        return Self::new(
            Vec4::new(self.x.x, self.y.x, self.z.x, self.w_axis.x),
            Vec4::new(self.x.y, self.y.y, self.z.y, self.w_axis.y),
            Vec4::new(self.x.z, self.y.z, self.z.z, self.w_axis.z),
            Vec4::new(self.x.w, self.y.w, self.z.w, self.w_axis.w),
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

        // Use cofactor expansion — compute each element of the adjugate matrix
        let m = |row: usize, col: usize| -> T {
            match (row, col) {
                (0, 0) => return self.x.x,
                (1, 0) => return self.x.y,
                (2, 0) => return self.x.z,
                (3, 0) => return self.x.w,
                (0, 1) => return self.y.x,
                (1, 1) => return self.y.y,
                (2, 1) => return self.y.z,
                (3, 1) => return self.y.w,
                (0, 2) => return self.z.x,
                (1, 2) => return self.z.y,
                (2, 2) => return self.z.z,
                (3, 2) => return self.z.w,
                (0, 3) => return self.w_axis.x,
                (1, 3) => return self.w_axis.y,
                (2, 3) => return self.w_axis.z,
                (3, 3) => return self.w_axis.w,
                _ => unreachable!(),
            }
        };

        // Cofactor C(i,j) = (-1)^(i+j) * M(i,j) where M is the 3x3 minor determinant
        let cofactor = |row: usize, col: usize| -> T {
            let mut rows = [0usize; 3];
            let mut cols = [0usize; 3];
            let mut ri = 0;
            let mut ci = 0;
            for k in 0..4 {
                if k != row {
                    rows[ri] = k;
                    ri += 1;
                }
                if k != col {
                    cols[ci] = k;
                    ci += 1;
                }
            }
            let det3 =
                  m(rows[0], cols[0]) * (m(rows[1], cols[1]) * m(rows[2], cols[2]) - m(rows[1], cols[2]) * m(rows[2], cols[1]))
                - m(rows[0], cols[1]) * (m(rows[1], cols[0]) * m(rows[2], cols[2]) - m(rows[1], cols[2]) * m(rows[2], cols[0]))
                + m(rows[0], cols[2]) * (m(rows[1], cols[0]) * m(rows[2], cols[1]) - m(rows[1], cols[1]) * m(rows[2], cols[0]));
            if (row + col) % 2 == 0 {
                return det3
            } else {
                return T::zero() - det3
            }
        };

        // Adjugate = transpose of cofactor matrix
        // inv[row][col] = cofactor(col, row) / det
        // Column c of the result: [cofactor(c,0), cofactor(c,1), cofactor(c,2), cofactor(c,3)] * inv_det
        return Some(Self::new(
            Vec4::new(cofactor(0, 0) * inv_det, cofactor(0, 1) * inv_det, cofactor(0, 2) * inv_det, cofactor(0, 3) * inv_det),
            Vec4::new(cofactor(1, 0) * inv_det, cofactor(1, 1) * inv_det, cofactor(1, 2) * inv_det, cofactor(1, 3) * inv_det),
            Vec4::new(cofactor(2, 0) * inv_det, cofactor(2, 1) * inv_det, cofactor(2, 2) * inv_det, cofactor(2, 3) * inv_det),
            Vec4::new(cofactor(3, 0) * inv_det, cofactor(3, 1) * inv_det, cofactor(3, 2) * inv_det, cofactor(3, 3) * inv_det),
        ))
    }

    /// Creates a 4x4 translation matrix.
    pub fn from_translation(translation: Vec3<T>) -> Self
    where T: Zero + One {
        return Self::new(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(translation.x, translation.y, translation.z, T::one()),
        )
    }

    /// Creates a 4x4 non-uniform scale matrix.
    pub fn from_scale(scale: Vec3<T>) -> Self
    where T: Zero + One {
        return Self::new(
            Vec4::new(scale.x, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), scale.y, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), scale.z, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a 4x4 rotation matrix around the X axis.
    pub fn from_rotation_x(angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        return Self::new(
            Vec4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), c, s, T::zero()),
            Vec4::new(T::zero(), -s, c, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a 4x4 rotation matrix around the Y axis.
    pub fn from_rotation_y(angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        return Self::new(
            Vec4::new(c, T::zero(), -s, T::zero()),
            Vec4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vec4::new(s, T::zero(), c, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a 4x4 rotation matrix around the Z axis.
    pub fn from_rotation_z(angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        return Self::new(
            Vec4::new(c, s, T::zero(), T::zero()),
            Vec4::new(-s, c, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a 4x4 rotation matrix from an axis and angle.
    pub fn from_axis_angle(axis: Vec3<T>, angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let (s, c) = angle.into().inner().sin_cos();
        let one_minus_c = T::one() - c;
        let ax = axis.normalize();

        return Self::new(
            Vec4::new(
                c + ax.x * ax.x * one_minus_c,
                ax.y * ax.x * one_minus_c + ax.z * s,
                ax.z * ax.x * one_minus_c - ax.y * s,
                T::zero(),
            ),
            Vec4::new(
                ax.x * ax.y * one_minus_c - ax.z * s,
                c + ax.y * ax.y * one_minus_c,
                ax.z * ax.y * one_minus_c + ax.x * s,
                T::zero(),
            ),
            Vec4::new(
                ax.x * ax.z * one_minus_c + ax.y * s,
                ax.y * ax.z * one_minus_c - ax.x * s,
                c + ax.z * ax.z * one_minus_c,
                T::zero(),
            ),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a 4x4 matrix from a 3x3 matrix, extending with a w identity column/row.
    pub fn from_mat3(m: Mat3<T>) -> Self
    where T: Zero + One {
        return Self::new(
            Vec4::new(m.x.x, m.x.y, m.x.z, T::zero()),
            Vec4::new(m.y.x, m.y.y, m.y.z, T::zero()),
            Vec4::new(m.z.x, m.z.y, m.z.z, T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    /// Creates a left-handed perspective projection matrix with a depth range of 0 to 1.
    /// `fov_y` is the vertical field of view, `aspect_ratio` is width/height,
    /// `z_near` and `z_far` are the near and far clip planes.
    pub fn perspective_lh(fov_y: impl Into<Rad<T>>, aspect_ratio: T, z_near: T, z_far: T) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let (s, c) = (fov_y.into().inner() / two).sin_cos();
        let h = c / s;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);

        return Self::new(
            Vec4::new(w, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), h, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), r, T::one()),
            Vec4::new(T::zero(), T::zero(), -r * z_near, T::zero()),
        )
    }

    /// Creates a right-handed perspective projection matrix with a depth range of 0 to 1.
    /// `fov_y` is the vertical field of view, `aspect_ratio` is width/height,
    /// `z_near` and `z_far` are the near and far clip planes.
    pub fn perspective_rh(fov_y: impl Into<Rad<T>>, aspect_ratio: T, z_near: T, z_far: T) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let half_fov = fov_y.into().inner() / two;
        let f = T::one() / half_fov.tan();
        let range = z_far - z_near;

        return Self::new(
            Vec4::new(f / aspect_ratio, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), f, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), -z_far / range, -T::one()),
            Vec4::new(T::zero(), T::zero(), -(z_far * z_near) / range, T::zero()),
        )
    }

    /// Creates a left-handed infinite reverse-Z perspective projection matrix.
    /// This uses a depth range of 1 (near) to 0 (far infinity).
    pub fn perspective_infinite_reverse_lh(fov_y: impl Into<Rad<T>>, aspect_ratio: T, z_near: T) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let (s, c) = (fov_y.into().inner() / two).sin_cos();
        let h = c / s;
        let w = h / aspect_ratio;

        return Self::new(
            Vec4::new(w, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), h, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), T::one()),
            Vec4::new(T::zero(), T::zero(), z_near, T::zero()),
        )
    }

    /// Creates a right-handed infinite reverse-Z perspective projection matrix.
    /// This uses a depth range of 1 (near) to 0 (far infinity).
    pub fn perspective_infinite_reverse_rh(fov_y: impl Into<Rad<T>>, aspect_ratio: T, z_near: T) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let half_fov = fov_y.into().inner() / two;
        let f = T::one() / half_fov.tan();

        return Self::new(
            Vec4::new(f / aspect_ratio, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), f, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::zero(), -T::one()),
            Vec4::new(T::zero(), T::zero(), z_near, T::zero()),
        )
    }

    /// Creates a left-handed orthographic projection matrix.
    pub fn orthographic_lh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let rml = right - left;
        let tmb = top - bottom;
        let r = T::one() / (far - near);

        return Self::new(
            Vec4::new(two / rml, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), two / tmb, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), r, T::zero()),
            Vec4::new(
                -(right + left) / rml,
                -(top + bottom) / tmb,
                -r * near,
                T::one(),
            ),
        )
    }

    /// Creates a right-handed orthographic projection matrix.
    pub fn orthographic_rh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let rml = right - left;
        let tmb = top - bottom;
        let fmn = far - near;

        return Self::new(
            Vec4::new(two / rml, T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), two / tmb, T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), -T::one() / fmn, T::zero()),
            Vec4::new(
                -(right + left) / rml,
                -(top + bottom) / tmb,
                -near / fmn,
                T::one(),
            ),
        )
    }

    /// Creates a right-handed look-at view matrix.
    /// `eye` is the camera position, `center` is the target point, `up` is the world up direction.
    pub fn look_at_rh(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_rh(eye, (center - eye).normalize(), up)
    }

    /// Creates a left-handed look-at view matrix.
    pub fn look_at_lh(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_lh(eye, (center - eye).normalize(), up)
    }

    /// Creates a right-handed view matrix using a camera position, a facing direction
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    pub fn look_to_rh(eye: Vec3<T>, dir: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        let f = dir;
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        return Self::new(
            Vec4::new(s.x, u.x, -f.x, T::zero()),
            Vec4::new(s.y, u.y, -f.y, T::zero()),
            Vec4::new(s.z, u.z, -f.z, T::zero()),
            Vec4::new(-s.dot(eye), -u.dot(eye), f.dot(eye), T::one()),
        )
    }

    /// Creates a left-handed view matrix using a camera position, a facing direction
    /// and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    pub fn look_to_lh(eye: Vec3<T>, dir: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_rh(eye, -dir, up)
    }

    /// Multiplies this matrix by a column vector, returning the resulting vector.
    pub fn mul_vec4(self, rhs: Vec4<T>) -> Vec4<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Vec4::new(
            self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z + self.w_axis.x * rhs.w,
            self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z + self.w_axis.y * rhs.w,
            self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z + self.w_axis.z * rhs.w,
            self.x.w * rhs.x + self.y.w * rhs.y + self.z.w * rhs.z + self.w_axis.w * rhs.w,
        )
    }

    /// Multiplies this matrix by another matrix.
    pub fn mul_mat4(self, rhs: Mat4<T>) -> Mat4<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return Mat4::new(
            self.mul_vec4(rhs.x),
            self.mul_vec4(rhs.y),
            self.mul_vec4(rhs.z),
            self.mul_vec4(rhs.w_axis),
        )
    }

    /// Transform a 3D point by this matrix (assumes w=1, performs perspective divide).
    pub fn transform_point3(self, point: Vec3<T>) -> Vec3<T>
    where T: FloatCore {
        let v = self.mul_vec4(Vec4::new(point.x, point.y, point.z, T::one()));
        let w_inv = v.w.recip();
        return Vec3::new(v.x * w_inv, v.y * w_inv, v.z * w_inv)
    }

    /// Transform a 3D vector by this matrix (assumes w=0, no translation).
    pub fn transform_vector3(self, vector: Vec3<T>) -> Vec3<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> + Zero {
        let v = self.mul_vec4(Vec4::new(vector.x, vector.y, vector.z, T::zero()));
        return Vec3::new(v.x, v.y, v.z)
    }

    /// Returns the matrix as a flat array in column-major order.
    pub fn to_cols_array(self) -> [T; 16] {
        return [
            self.x.x, self.x.y, self.x.z, self.x.w,
            self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w,
            self.w_axis.x, self.w_axis.y, self.w_axis.z, self.w_axis.w,
        ]
    }

    /// Creates a matrix from a flat array in column-major order.
    pub fn from_cols_array(arr: [T; 16]) -> Self
    where T: Copy {
        return Self::new(
            Vec4::new(arr[0], arr[1], arr[2], arr[3]),
            Vec4::new(arr[4], arr[5], arr[6], arr[7]),
            Vec4::new(arr[8], arr[9], arr[10], arr[11]),
            Vec4::new(arr[12], arr[13], arr[14], arr[15]),
        )
    }

    /// Returns a reference to a column by index.
    pub fn col(&self, index: usize) -> &Vec4<T> {
        match index {
            0 => return &self.x,
            1 => return &self.y,
            2 => return &self.z,
            3 => return &self.w_axis,
            _ => panic!("index out of bounds: Mat4 has 4 columns but index is {}", index),
        }
    }

    /// Returns a mutable reference to a column by index.
    pub fn col_mut(&mut self, index: usize) -> &mut Vec4<T> {
        match index {
            0 => return &mut self.x,
            1 => return &mut self.y,
            2 => return &mut self.z,
            3 => return &mut self.w_axis,
            _ => panic!("index out of bounds: Mat4 has 4 columns but index is {}", index),
        }
    }

    /// Returns a row by index.
    pub fn row(&self, index: usize) -> Vec4<T>
    where T: Copy {
        match index {
            0 => return Vec4::new(self.x.x, self.y.x, self.z.x, self.w_axis.x),
            1 => return Vec4::new(self.x.y, self.y.y, self.z.y, self.w_axis.y),
            2 => return Vec4::new(self.x.z, self.y.z, self.z.z, self.w_axis.z),
            3 => return Vec4::new(self.x.w, self.y.w, self.z.w, self.w_axis.w),
            _ => panic!("index out of bounds: Mat4 has 4 rows but index is {}", index),
        }
    }

    /// Returns true if any element is NaN.
    pub fn is_nan(self) -> bool
    where T: FloatCore {
        return self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w_axis.is_nan()
    }

    /// Returns true if all elements are finite.
    pub fn is_finite(self) -> bool
    where T: FloatCore {
        return self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w_axis.is_finite()
    }
}

impl<T: fmt::Display + Copy> fmt::Display for Mat4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Mat4([{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}])",
            self.x.x, self.y.x, self.z.x, self.w_axis.x,
            self.x.y, self.y.y, self.z.y, self.w_axis.y,
            self.x.z, self.y.z, self.z.z, self.w_axis.z,
            self.x.w, self.y.w, self.z.w, self.w_axis.w)
    }
}

impl<T> Index<usize> for Mat4<T> {
    type Output = Vec4<T>;

    fn index(&self, index: usize) -> &Self::Output {
        return self.col(index)
    }
}

impl<T> IndexMut<usize> for Mat4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.col_mut(index)
    }
}

// Mat4 * Mat4
impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Mat4<T>> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        return self.mul_mat4(rhs)
    }
}

// Mat4 *= Mat4
impl<T: Copy + Mul<Output = T> + Add<Output = T>> MulAssign<Mat4<T>> for Mat4<T> {
    fn mul_assign(&mut self, rhs: Mat4<T>) {
        *self = self.mul_mat4(rhs);
    }
}

// Mat4 * Vec4
impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        return self.mul_vec4(rhs)
    }
}

// Mat4 * scalar
impl<T: Copy + Mul<Output = T>> Mul<T> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Mat4::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w_axis * rhs,
        )
    }
}

// Mat4 *= scalar
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Mat4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w_axis = self.w_axis * rhs;
    }
}

// Mat4 / scalar
impl<T: Copy + Div<Output = T>> Div<T> for Mat4<T> {
    type Output = Mat4<T>;

    fn div(self, rhs: T) -> Self::Output {
        return Mat4::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w_axis / rhs,
        )
    }
}

// Mat4 /= scalar
impl<T: Copy + Div<Output = T>> DivAssign<T> for Mat4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self.w_axis = self.w_axis / rhs;
    }
}

// Mat4 + Mat4
impl<T: Copy + Add<Output = T>> Add<Mat4<T>> for Mat4<T> {
    type Output = Mat4<T>;

    fn add(self, rhs: Mat4<T>) -> Self::Output {
        return Mat4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w_axis + rhs.w_axis,
        )
    }
}

// Mat4 += Mat4
impl<T: Copy + Add<Output = T>> AddAssign<Mat4<T>> for Mat4<T> {
    fn add_assign(&mut self, rhs: Mat4<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w_axis = self.w_axis + rhs.w_axis;
    }
}

// Mat4 - Mat4
impl<T: Copy + Sub<Output = T>> Sub<Mat4<T>> for Mat4<T> {
    type Output = Mat4<T>;

    fn sub(self, rhs: Mat4<T>) -> Self::Output {
        return Mat4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w_axis - rhs.w_axis,
        )
    }
}

// Mat4 -= Mat4
impl<T: Copy + Sub<Output = T>> SubAssign<Mat4<T>> for Mat4<T> {
    fn sub_assign(&mut self, rhs: Mat4<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w_axis = self.w_axis - rhs.w_axis;
    }
}

// -Mat4
impl<T: Neg<Output = T>> Neg for Mat4<T> {
    type Output = Mat4<T>;

    fn neg(self) -> Self::Output {
        return Mat4::new(-self.x, -self.y, -self.z, -self.w_axis)
    }
}

// [Vec4<T>; 4] -> Mat4<T>
impl<T> From<[Vec4<T>; 4]> for Mat4<T> {
    fn from([x, y, z, w]: [Vec4<T>; 4]) -> Self {
        return Self::new(x, y, z, w)
    }
}

// Mat4<T> -> [Vec4<T>; 4]
impl<T> From<Mat4<T>> for [Vec4<T>; 4] {
    fn from(m: Mat4<T>) -> Self {
        return [m.x, m.y, m.z, m.w_axis]
    }
}

// Mat3<T> -> Mat4<T>
impl<T: Zero + One> From<Mat3<T>> for Mat4<T> {
    fn from(m: Mat3<T>) -> Self {
        return Self::from_mat3(m)
    }
}
