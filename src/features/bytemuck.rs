use crate::{Vec2, Vec3, Vec4};
use crate::{Mat2, Mat3, Mat4};
use bytemuck::{Pod, Zeroable};

unsafe impl<T: Pod> Pod for Vec2<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec2<T> {}

unsafe impl<T: Pod> Pod for Vec3<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec3<T> {}

unsafe impl<T: Pod> Pod for Vec4<T> {}
unsafe impl<T: Zeroable> Zeroable for Vec4<T> {}

unsafe impl<T: Pod> Pod for Mat2<T> {}
unsafe impl<T: Zeroable> Zeroable for Mat2<T> {}

unsafe impl<T: Pod> Pod for Mat3<T> {}
unsafe impl<T: Zeroable> Zeroable for Mat3<T> {}

unsafe impl<T: Pod> Pod for Mat4<T> {}
unsafe impl<T: Zeroable> Zeroable for Mat4<T> {}
