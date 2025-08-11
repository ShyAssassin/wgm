use super::Vec2;

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A 2x2 column major matrix of type T.
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

impl<T> Mat2<T> {
    /// Create a new `Mat2<T>` with the given x and y vectors.
    pub const fn new(x: Vec2<T>, y: Vec2<T>) -> Self {
        return Self { x, y }
    }
}
