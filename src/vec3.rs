use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

unsafe impl<T: Pod> Pod for Vec3<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec3<T> {}
