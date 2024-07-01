use super::Vec3;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Mat3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

unsafe impl<T: Pod> Pod for Mat3<T> {}
unsafe impl<T: Zeroable> Zeroable for Mat3<T> {}
