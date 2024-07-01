use super::Vec4;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

unsafe impl<T: Pod> Pod for Mat4<T> {}
unsafe impl<T: Zeroable> Zeroable for Mat4<T> {}
