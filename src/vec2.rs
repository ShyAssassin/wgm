use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

unsafe impl<T: Pod> Pod for Vec2<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec2<T> {}
