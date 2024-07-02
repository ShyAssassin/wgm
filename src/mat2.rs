use super::Vec2;

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

impl<T> Mat2<T> {
    pub const fn new(x: Vec2<T>, y: Vec2<T>) -> Self {
        Self { x, y }
    }
}
