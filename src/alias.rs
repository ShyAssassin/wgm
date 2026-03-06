use super::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4, Quat};

/// Creates a new `Vec2<T>`.
pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    return Vec2::new(x, y)
}

/// Creates a new `Vec3<T>`.
pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    return Vec3::new(x, y, z)
}

/// Creates a new `Vec4<T>`.
pub fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    return Vec4::new(x, y, z, w)
}

/// Creates a new `Mat2<T>` from two column vectors.
pub fn mat2x2<T>(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
    return Mat2::new(x, y)
}

/// Creates a new `Mat3<T>` from three column vectors.
pub fn mat3x3<T>(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
    return Mat3::new(x, y, z)
}

/// Creates a new `Mat4<T>` from four column vectors.
pub fn mat4x4<T>(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w_axis: Vec4<T>) -> Mat4<T> {
    return Mat4::new(x, y, z, w_axis)
}

/// Creates a new `Quat<T>`.
pub fn quat<T>(x: T, y: T, z: T, w: T) -> Quat<T> {
    return Quat::new(x, y, z, w)
}

// f32 type aliases
pub type Vec2f32 = Vec2<f32>;
pub type Vec3f32 = Vec3<f32>;
pub type Vec4f32 = Vec4<f32>;
pub type Mat2f32 = Mat2<f32>;
pub type Mat3f32 = Mat3<f32>;
pub type Mat4f32 = Mat4<f32>;
pub type Quatf32 = Quat<f32>;

// f64 type aliases
pub type Vec2f64 = Vec2<f64>;
pub type Vec3f64 = Vec3<f64>;
pub type Vec4f64 = Vec4<f64>;
pub type Mat2f64 = Mat2<f64>;
pub type Mat3f64 = Mat3<f64>;
pub type Mat4f64 = Mat4<f64>;
pub type Quatf64 = Quat<f64>;

// i32 type aliases
pub type Vec2i32 = Vec2<i32>;
pub type Vec3i32 = Vec3<i32>;
pub type Vec4i32 = Vec4<i32>;

// u32 type aliases
pub type Vec2u32 = Vec2<u32>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec4u32 = Vec4<u32>;

// i64 type aliases
pub type Vec2i64 = Vec2<i64>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec4i64 = Vec4<i64>;

// u64 type aliases
pub type Vec2u64 = Vec2<u64>;
pub type Vec3u64 = Vec3<u64>;
pub type Vec4u64 = Vec4<u64>;

// usize type aliases
pub type Vec2usize = Vec2<usize>;
pub type Vec3usize = Vec3<usize>;
pub type Vec4usize = Vec4<usize>;
