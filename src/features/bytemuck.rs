use num_traits::Float;
use bytemuck::{Pod, Zeroable};
use crate::{Vec2, Vec3, Vec4};
use crate::{Mat2, Mat3, Mat4};
use crate::units::{Deg, Rad, Tau};

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

unsafe impl<T: Pod + Float> Pod for Deg<T> {}
unsafe impl<T: Zeroable + Float> Zeroable for Deg<T> {}

unsafe impl<T: Pod + Float> Pod for Rad<T> {}
unsafe impl<T: Zeroable + Float> Zeroable for Rad<T> {}

unsafe impl<T: Pod + Float> Pod for Tau<T> {}
unsafe impl<T: Zeroable + Float> Zeroable for Tau<T> {}
