use super::Vec2;

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}
