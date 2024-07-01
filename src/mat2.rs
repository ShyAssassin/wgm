use super::Vec2;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

unsafe impl<T: Pod> Pod for Mat2<T> {}
unsafe impl<T: Zeroable> Zeroable for Mat2<T> {}
