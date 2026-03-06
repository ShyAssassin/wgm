use core::fmt;
use crate::units::Rad;
use super::{Vec3, Vec4, Mat3, Mat4};
use core::ops::{Add, Sub, Mul, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign};
use num_traits::{Float, One, Zero, NumCast, float::FloatCore};

/// A quaternion of type T, representing a rotation in 3D space.
///
/// Stored as `(x, y, z, w)` where the scalar part is `w` and the vector part is `(x, y, z)`.
/// The identity quaternion is `Quat { x: 0, y: 0, z: 0, w: 1 }`.
#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Quat<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Quat<T> {
    /// Create a new `Quat<T>` from the given x, y, z, w values.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        return Self { x, y, z, w }
    }

    /// Returns the identity quaternion (no rotation).
    pub fn identity() -> Self
    where T: Zero + One {
        return Self::new(T::zero(), T::zero(), T::zero(), T::one())
    }

    /// Creates a quaternion from an axis and angle.
    /// The axis should be a unit vector.
    pub fn from_axis_angle(axis: Vec3<T>, angle: impl Into<Rad<T>>) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let half_angle = angle.into().inner() / two;
        let (s, c) = half_angle.sin_cos();
        let a = axis.normalize();
        return Self::new(a.x * s, a.y * s, a.z * s, c)
    }

    /// Creates a quaternion from Euler angles.
    /// Rotation order is ZYX (yaw, pitch, roll).
    pub fn from_euler_angles(roll: impl Into<Rad<T>>, pitch: impl Into<Rad<T>>, yaw: impl Into<Rad<T>>) -> Self
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let (sr, cr) = (roll.into().inner() / two).sin_cos();
        let (sp, cp) = (pitch.into().inner() / two).sin_cos();
        let (sy, cy) = (yaw.into().inner() / two).sin_cos();

        return Self::new(
            sr * cp * cy - cr * sp * sy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
            cr * cp * cy + sr * sp * sy,
        )
    }

    /// Creates a quaternion from a 3x3 rotation matrix.
    pub fn from_rotation_mat3(m: Mat3<T>) -> Self
    where T: Float {
        let one: T = T::one();
        let zero: T = T::zero();
        let quarter: T = NumCast::from(0.25f64).unwrap();

        let trace = m.x.x + m.y.y + m.z.z;

        if trace > zero {
            let s = (trace + one).sqrt() * NumCast::from(2.0f64).unwrap();
            let inv_s = one / s;
            return Self::new(
                (m.y.z - m.z.y) * inv_s,
                (m.z.x - m.x.z) * inv_s,
                (m.x.y - m.y.x) * inv_s,
                quarter * s,
            )
        } else if m.x.x > m.y.y && m.x.x > m.z.z {
            let s = (one + m.x.x - m.y.y - m.z.z).sqrt() * NumCast::from(2.0f64).unwrap();
            let inv_s = one / s;
            return Self::new(
                quarter * s,
                (m.y.x + m.x.y) * inv_s,
                (m.z.x + m.x.z) * inv_s,
                (m.y.z - m.z.y) * inv_s,
            )
        } else if m.y.y > m.z.z {
            let s = (one + m.y.y - m.x.x - m.z.z).sqrt() * NumCast::from(2.0f64).unwrap();
            let inv_s = one / s;
            return Self::new(
                (m.y.x + m.x.y) * inv_s,
                quarter * s,
                (m.z.y + m.y.z) * inv_s,
                (m.z.x - m.x.z) * inv_s,
            )
        } else {
            let s = (one + m.z.z - m.x.x - m.y.y).sqrt() * NumCast::from(2.0f64).unwrap();
            let inv_s = one / s;
            return Self::new(
                (m.z.x + m.x.z) * inv_s,
                (m.z.y + m.y.z) * inv_s,
                quarter * s,
                (m.x.y - m.y.x) * inv_s,
            )
        }
    }

    /// Creates a quaternion rotation from a facing direction and an up direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    pub fn look_to_rh(dir: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::from_rotation_mat3(Mat3::look_to_rh(dir, up))
    }

    /// Creates a quaternion rotation from a facing direction and an up direction.
    ///
    /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    pub fn look_to_lh(dir: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_rh(-dir, up)
    }

    /// Creates a quaternion from a camera position, a focal point and an up direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    pub fn look_at_rh(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_rh((center - eye).normalize(), up)
    }

    /// Creates a quaternion from a camera position, a focal point and an up direction.
    ///
    /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    pub fn look_at_lh(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self
    where T: Float {
        return Self::look_to_lh((center - eye).normalize(), up)
    }

    /// Returns the length (magnitude) of the quaternion.
    pub fn length(self) -> T
    where T: Float {
        return self.length_squared().sqrt()
    }

    /// Returns the squared length of the quaternion.
    pub fn length_squared(self) -> T
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Returns the normalized quaternion (unit quaternion).
    pub fn normalize(self) -> Self
    where T: Float {
        let len = self.length();
        if len.is_zero() {
            return self
        }
        let inv = len.recip();
        return Self::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
    }

    /// Returns true if the quaternion is normalized (unit length within epsilon).
    pub fn is_normalized(self) -> bool
    where T: FloatCore {
        let epsilon: T = NumCast::from(1e-6f64).unwrap();
        return (self.length_squared() - T::one()).abs() < epsilon
    }

    /// Returns the conjugate of the quaternion.
    pub fn conjugate(self) -> Self
    where T: Neg<Output = T> {
        return Self::new(-self.x, -self.y, -self.z, self.w)
    }

    /// Returns the inverse of the quaternion.
    /// For unit quaternions, the inverse equals the conjugate.
    pub fn inverse(self) -> Self
    where T: FloatCore {
        let len_sq = self.length_squared();
        if len_sq.is_zero() {
            return self
        }
        let inv = len_sq.recip();
        return Self::new(-self.x * inv, -self.y * inv, -self.z * inv, self.w * inv)
    }

    /// Returns the dot product of two quaternions.
    pub fn dot(self, rhs: Quat<T>) -> T
    where T: Copy + Mul<Output = T> + Add<Output = T> {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    /// Linearly interpolates between two quaternions and normalizes the result.
    pub fn lerp(self, other: Quat<T>, t: T) -> Self
    where T: Float {
        let one_minus_t = T::one() - t;
        let result = Self::new(
            self.x * one_minus_t + other.x * t,
            self.y * one_minus_t + other.y * t,
            self.z * one_minus_t + other.z * t,
            self.w * one_minus_t + other.w * t,
        );
        return result.normalize()
    }

    /// Spherical linear interpolation between two quaternions.
    pub fn slerp(self, other: Quat<T>, t: T) -> Self
    where T: Float {
        let mut dot = self.dot(other);

        // If the dot product is negative, negate one to take the shorter path
        let other = if dot < T::zero() {
            dot = T::zero() - dot;
            Self::new(-other.x, -other.y, -other.z, -other.w)
        } else {
            other
        };

        let threshold: T = NumCast::from(0.9995f64).unwrap();

        if dot > threshold {
            // Quaternions are close — use lerp to avoid division by sin(~0)
            return self.lerp(other, t)
        }

        let theta = dot.acos();
        let sin_theta = theta.sin();
        let inv_sin = sin_theta.recip();
        let s0 = ((T::one() - t) * theta).sin() * inv_sin;
        let s1 = (t * theta).sin() * inv_sin;

        return Self::new(
            self.x * s0 + other.x * s1,
            self.y * s0 + other.y * s1,
            self.z * s0 + other.z * s1,
            self.w * s0 + other.w * s1,
        )
    }

    /// Rotates a 3D vector by this quaternion.
    pub fn mul_vec3(self, v: Vec3<T>) -> Vec3<T>
    where T: Copy + NumCast + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        // q * v * q^-1 using the optimized formula:
        // t = 2 * cross(q.xyz, v)
        // result = v + q.w * t + cross(q.xyz, t)
        let two: T = NumCast::from(2.0f64).unwrap();
        let qvec = Vec3::new(self.x, self.y, self.z);
        let t = qvec.cross(v) * two;
        return v + t * self.w + qvec.cross(t)
    }

    /// Multiplies two quaternions (composes rotations).
    pub fn mul_quat(self, rhs: Quat<T>) -> Quat<T>
    where T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        return Quat::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }

    /// Converts this quaternion to a 3x3 rotation matrix.
    pub fn to_mat3(self) -> Mat3<T>
    where T: Copy + NumCast + One + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        let two: T = NumCast::from(2.0f64).unwrap();
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let yz = self.y * self.z;
        let wx = self.w * self.x;
        let wy = self.w * self.y;
        let wz = self.w * self.z;

        return Mat3::new(
            Vec3::new(
                T::one() - two * (y2 + z2),
                two * (xy + wz),
                two * (xz - wy),
            ),
            Vec3::new(
                two * (xy - wz),
                T::one() - two * (x2 + z2),
                two * (yz + wx),
            ),
            Vec3::new(
                two * (xz + wy),
                two * (yz - wx),
                T::one() - two * (x2 + y2),
            ),
        )
    }

    /// Converts this quaternion to a 4x4 rotation matrix.
    pub fn to_mat4(self) -> Mat4<T>
    where T: Copy + NumCast + Zero + One + Mul<Output = T> + Add<Output = T> + Sub<Output = T> {
        return Mat4::from_mat3(self.to_mat3())
    }

    /// Returns the angle between this quaternion and `other`, as [`Rad<T>`].
    pub fn angle_between(self, other: Quat<T>) -> Rad<T>
    where T: Float {
        let d = self.dot(other).abs();
        let one: T = T::one();
        if d >= one {
            return Rad(T::zero())
        }
        let two: T = NumCast::from(2.0f64).unwrap();
        return Rad(two * d.acos())
    }

    /// Returns the rotation axis and angle of this quaternion.
    /// The angle is returned as [`Rad<T>`].
    pub fn to_axis_angle(self) -> (Vec3<T>, Rad<T>)
    where T: Float {
        let two: T = NumCast::from(2.0f64).unwrap();
        let n = self.normalize();
        let angle = Rad(two * n.w.acos());
        let sin_half = (T::one() - n.w * n.w).sqrt();
        let epsilon: T = NumCast::from(1e-6f64).unwrap();
        if sin_half < epsilon {
            return (Vec3::new(T::one(), T::zero(), T::zero()), angle)
        }
        let inv = sin_half.recip();
        return (Vec3::new(n.x * inv, n.y * inv, n.z * inv), angle)
    }

    /// Returns the vector part of the quaternion as a Vec3.
    pub fn vector(self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.z)
    }

    /// Returns the scalar part of the quaternion.
    pub fn scalar(self) -> T {
        return self.w
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

    /// Converts to a Vec4.
    pub fn to_vec4(self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.w)
    }

    /// Creates a quaternion from a Vec4 (x, y, z, w).
    pub fn from_vec4(v: Vec4<T>) -> Self {
        return Self::new(v.x, v.y, v.z, v.w)
    }
}

impl<T: fmt::Display> fmt::Display for Quat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Quat({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// Quat * Quat (composition)
impl<T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> Mul<Quat<T>> for Quat<T> {
    type Output = Quat<T>;

    fn mul(self, rhs: Quat<T>) -> Self::Output {
        return self.mul_quat(rhs)
    }
}

// Quat *= Quat
impl<T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> MulAssign<Quat<T>> for Quat<T> {
    fn mul_assign(&mut self, rhs: Quat<T>) {
        *self = self.mul_quat(rhs);
    }
}

// Quat * Vec3 (rotate vector)
impl<T: Copy + NumCast + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> Mul<Vec3<T>> for Quat<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        return self.mul_vec3(rhs)
    }
}

// Quat * scalar
impl<T: Copy + Mul<Output = T>> Mul<T> for Quat<T> {
    type Output = Quat<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Quat::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

// Quat *= scalar
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Quat<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

// Quat + Quat
impl<T: Copy + Add<Output = T>> Add<Quat<T>> for Quat<T> {
    type Output = Quat<T>;

    fn add(self, rhs: Quat<T>) -> Self::Output {
        return Quat::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

// Quat += Quat
impl<T: Copy + Add<Output = T>> AddAssign<Quat<T>> for Quat<T> {
    fn add_assign(&mut self, rhs: Quat<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

// Quat - Quat
impl<T: Copy + Sub<Output = T>> Sub<Quat<T>> for Quat<T> {
    type Output = Quat<T>;

    fn sub(self, rhs: Quat<T>) -> Self::Output {
        return Quat::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

// Quat -= Quat
impl<T: Copy + Sub<Output = T>> SubAssign<Quat<T>> for Quat<T> {
    fn sub_assign(&mut self, rhs: Quat<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

// -Quat
impl<T: Neg<Output = T>> Neg for Quat<T> {
    type Output = Quat<T>;

    fn neg(self) -> Self::Output {
        return Quat::new(-self.x, -self.y, -self.z, -self.w)
    }
}

// [T; 4] -> Quat<T>
impl<T> From<[T; 4]> for Quat<T> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        return Self::new(x, y, z, w)
    }
}

// (T, T, T, T) -> Quat<T>
impl<T> From<(T, T, T, T)> for Quat<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        return Self::new(x, y, z, w)
    }
}

// Quat<T> -> [T; 4]
impl<T> From<Quat<T>> for [T; 4] {
    fn from(q: Quat<T>) -> Self {
        return [q.x, q.y, q.z, q.w]
    }
}

// Quat<T> -> (T, T, T, T)
impl<T> From<Quat<T>> for (T, T, T, T) {
    fn from(q: Quat<T>) -> Self {
        return (q.x, q.y, q.z, q.w)
    }
}

// Vec4<T> -> Quat<T>
impl<T> From<Vec4<T>> for Quat<T> {
    fn from(v: Vec4<T>) -> Self {
        return Self::new(v.x, v.y, v.z, v.w)
    }
}

// Quat<T> -> Vec4<T>
impl<T> From<Quat<T>> for Vec4<T> {
    fn from(q: Quat<T>) -> Self {
        return Self::new(q.x, q.y, q.z, q.w)
    }
}
