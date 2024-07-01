use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

unsafe impl<T: Pod> Pod for Vec4<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec4<T> {}
