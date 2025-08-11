use crate::Vec4;

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A 4x4 column major matrix of type T.
pub struct Mat4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl<T> Mat4<T> {
    /// Create a new `Mat3<T>` with the given x, y, z and w vectors.
    pub const fn new(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self {
        return Self { x, y, z, w }
    }
}
