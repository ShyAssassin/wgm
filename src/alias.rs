// TODO: Figure out if aliases are a good idea or not.
use super::{Vec2, Vec3, Vec4};
use super::{Mat2, Mat3, Mat4};

/// Create a new `Vec2<T>` from the given x and y values.
pub const fn vec2<T>(x: T, y: T) -> Vec2<T> {
    return Vec2::new(x, y)
}

/// Create a new `Vec3<T>::new()` from the given x, y, and z values.
pub const fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    return Vec3::new(x, y, z)
}

/// Create a new `Vec4<T>::new()` from the given x, y, z, and w values.
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    return Vec4::new(x, y, z, w)
}

/// Alias for `Mat2<T>::new()`. Create a new `Mat2<T>` from the given x and y vectors.
pub const fn mat2x2<T>(x: Vec2<T>, y: Vec2<T>) -> Mat2<T> {
    return Mat2::new(x, y)
}

/// Alias for `Mat3<T>::new()`. Create a new `Mat3<T>` from the given x, y, and z vectors.
pub const fn mat3x3<T>(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
    return Mat3::new(x, y, z)
}

/// Alias for `Mat4<T>::new()`. Create a new `Mat4<T>` from the given x, y, z, and w vectors.
pub const fn mat4x4<T>(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Mat4<T> {
    return Mat4::new(x, y, z, w)
}

/// Alias for `Vec2<f32>`.
pub type Vec2f32 = Vec2<f32>;
/// Alias for `Vec2<f64>`.
pub type Vec2f64 = Vec2<f64>;
/// Alias for `Vec2<i8>`.
pub type Vec2i8 = Vec2<i8>;
/// Alias for `Vec2<i16>`.
pub type Vec2i16 = Vec2<i16>;
/// Alias for `Vec2<i16>`.
pub type Vec2i32 = Vec2<i32>;
/// Alias for `Vec2<i64>`.
pub type Vec2i64 = Vec2<i64>;
/// Alias for `Vec2<u8>`.
pub type Vec2u8 = Vec2<u8>;
/// Alias for `Vec2<u16>`.
pub type Vec2u16 = Vec2<u16>;
/// Alias for `Vec2<u16>`.
pub type Vec2u32 = Vec2<u32>;
/// Alias for `Vec2<u64>`.
pub type Vec2u64 = Vec2<u64>;


/// Alias for `Vec3<f32>`.
pub type Vec3f32 = Vec3<f32>;
/// Alias for `Vec3<f64>`.
pub type Vec3f64 = Vec3<f64>;
/// Alias for `Vec3<i8>`.
pub type Vec3i8 = Vec3<i8>;
/// Alias for `Vec3<i16>`.
pub type Vec3i16 = Vec3<i16>;
/// Alias for `Vec3<i32>`.
pub type Vec3i32 = Vec3<i32>;
/// Alias for `Vec3<i64>`.
pub type Vec3i64 = Vec3<i64>;
/// Alias for `Vec3<u8>`.
pub type Vec3u8 = Vec3<u8>;
/// Alias for `Vec3<u16>`.
pub type Vec3u16 = Vec3<u16>;
/// Alias for `Vec3<u32>`.
pub type Vec3u32 = Vec3<u32>;
/// Alias for `Vec3<u64>`.
pub type Vec3u64 = Vec3<u64>;


/// Alias for `Vec4<f32>`.
pub type Vec4f32 = Vec4<f32>;
/// Alias for `Vec4<f64>`.
pub type Vec4f64 = Vec4<f64>;
/// Alias for `Vec4<i8>`.
pub type Vec4i8 = Vec4<i8>;
/// Alias for `Vec4<i16>`.
pub type Vec4i16 = Vec4<i16>;
/// Alias for `Vec4<i32>`.
pub type Vec4i32 = Vec4<i32>;
/// Alias for `Vec4<i64>`.
pub type Vec4i64 = Vec4<i64>;
/// Alias for `Vec4<u8>`.
pub type Vec4u8 = Vec4<u8>;
/// Alias for `Vec4<u16>`.
pub type Vec4u16 = Vec4<u16>;
/// Alias for `Vec4<u32>`.
pub type Vec4u32 = Vec4<u32>;
/// Alias for `Vec4<u64>`.
pub type Vec4u64 = Vec4<u64>;
