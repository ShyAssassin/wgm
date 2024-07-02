use super::{Vec2, Vec3};
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

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

impl<T> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub const fn splat(v: T) -> Self
    where T: Copy {
        Self::new(v, v, v, v)
    }

    pub fn truncate(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

// Vec4<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

// Vec4<T> *= T
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

// Vec4<T> / T
impl<T: Copy + Div<Output = T>> Div<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

// Vec4<T> /= T
impl<T: Copy + Div<Output = T>> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self.w = self.w / rhs;
    }
}

// Vec4<T> + T
impl<T: Copy + Add<Output = T>> Add<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec4::new(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs)
    }
}

// Vec4<T> += T
impl<T: Copy + Add<Output = T>> AddAssign<T> for Vec4<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
        self.w = self.w + rhs;
    }
}

// Vec4<T> - T
impl<T: Copy + Sub<Output = T>> Sub<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec4::new(self.x - rhs, self.y - rhs, self.z - rhs, self.w - rhs)
    }
}

// Vec4<T> -= T
impl<T: Copy + Sub<Output = T>> SubAssign<T> for Vec4<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
        self.w = self.w - rhs;
    }
}

// Vec4<T> * Vec4<T>
impl<T: Copy + Mul<Output = T>> Mul<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }
}

// Vec4<T> *= Vec4<T>
impl<T: Copy + Mul<Output = T>> MulAssign<Vec4<T>> for Vec4<T> {
    fn mul_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self.w = self.w * rhs.w;
    }
}

// Vec4<T> / Vec4<T>
impl<T: Copy + Div<Output = T>> Div<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn div(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
    }
}

// Vec4<T> /= Vec4<T>
impl<T: Copy + Div<Output = T>> DivAssign<Vec4<T>> for Vec4<T> {
    fn div_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self.w = self.w / rhs.w;
    }
}

// Vec4<T> + Vec4<T>
impl<T: Copy + Add<Output = T>> Add<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

// Vec4<T> += Vec4<T>
impl<T: Copy + Add<Output = T>> AddAssign<Vec4<T>> for Vec4<T> {
    fn add_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

// Vec4<T> - Vec4<T>
impl<T: Copy + Sub<Output = T>> Sub<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

// Vec4<T> -= Vec4<T>
impl<T: Copy + Sub<Output = T>> SubAssign<Vec4<T>> for Vec4<T> {
    fn sub_assign(&mut self, rhs: Vec4<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

// T -> Vec4<T>
impl<T: Copy> From<T> for Vec4<T>{
    fn from(v: T) -> Self {
        Self::new(v, v, v, v)
    }
}

// [T; 4] -> Vec4<T>
impl<T> From<[T; 4]> for Vec4<T> {
    fn from([x, y, z, w]: [T; 4]) -> Self {
        Self::new(x, y, z, w)
    }
}

// [Vec2<T>; 2] -> Vec4<T>
impl<T> From<[Vec2<T>; 2]> for Vec4<T> {
    fn from([v1, v2]: [Vec2<T>; 2]) -> Self {
        Self::new(v1.x, v1.y, v2.x, v2.y)
    }
}

// (T, T, T, T) -> Vec4<T>
impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self::new(x, y, z, w)
    }
}

// (Vec3<T>, T) -> Vec4<T>
impl<T> From<(Vec3<T>, T)> for Vec4<T> {
    fn from((v, t): (Vec3<T>, T)) -> Self {
        Self::new(v.x, v.y, v.z, t)
    }
}

// (T, Vec3<T>) -> Vec4<T>
impl<T> From<(T, Vec3<T>)> for Vec4<T> {
    fn from((t, v): (T, Vec3<T>)) -> Self {
        Self::new(t, v.x, v.y, v.z)
    }
}

// (Vec2<T>, T, T) -> Vec4<T>
impl<T> From<(Vec2<T>, T, T)> for Vec4<T> {
    fn from((v, z, w): (Vec2<T>, T, T)) -> Self {
        Self::new(v.x, v.y, z, w)
    }
}

// (T, T, Vec2<T>) -> Vec4<T>
impl<T> From<(T, T, Vec2<T>)> for Vec4<T> {
    fn from((t1, t2, v): (T, T, Vec2<T>)) -> Self {
        Self::new(t1, t2, v.x, v.y)
    }
}

// (T, Vec2<T>, T) -> Vec4<T>
impl<T> From<(T, Vec2<T>, T)> for Vec4<T> {
    fn from((t1, v, t2): (T, Vec2<T>, T)) -> Self {
        Self::new(t1, v.x, v.y, t2)
    }
}

// (Vec2<T>, Vec2<T>) -> Vec4<T>
impl<T> From<(Vec2<T>, Vec2<T>)> for Vec4<T> {
    fn from((v1, v2): (Vec2<T>, Vec2<T>)) -> Self {
        Self::new(v1.x, v1.y, v2.x, v2.y)
    }
}

// Vec4<T> -> [T; 4]
impl<T> From<Vec4<T>> for [T; 4] {
    fn from(v: Vec4<T>) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

// Vec4<T> -> [Vec2<T>; 2]
impl<T> From<Vec4<T>> for [Vec2<T>; 2] {
    fn from(v: Vec4<T>) -> Self {
        [Vec2::new(v.x, v.y), Vec2::new(v.z, v.w)]
    }
}

// Vec4<T> -> (T, T, T, T)
impl<T> From<Vec4<T>> for (T, T, T, T) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

// Vec4<T> -> (Vec2<T>, Vec2<T>)
impl<T> From<Vec4<T>> for (Vec2<T>, Vec2<T>) {
    fn from(v: Vec4<T>) -> Self {
        (Vec2::new(v.x, v.y), Vec2::new(v.z, v.w))
    }
}

// Vec4<T> -> (T, T, Vec2<T>)
impl<T> From<Vec4<T>> for (T, T, Vec2<T>) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, v.y, Vec2::new(v.z, v.w))
    }
}

// Vec4<T> -> (Vec2<T>, T, T)
impl<T> From<Vec4<T>> for (Vec2<T>, T, T) {
    fn from(v: Vec4<T>) -> Self {
        (Vec2::new(v.x, v.y), v.z, v.w)
    }
}

// Vec4<T> -> (T, Vec2<T>, T)
impl<T> From<Vec4<T>> for (T, Vec2<T>, T) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, Vec2::new(v.y, v.z), v.w)
    }
}

// Vec4<T> -> (Vec3<T>, T)
impl<T> From<Vec4<T>> for (Vec3<T>, T) {
    fn from(v: Vec4<T>) -> Self {
        (Vec3::new(v.x, v.y, v.z), v.w)
    }
}

// Vec4<T> -> (T, Vec3<T>)
impl<T> From<Vec4<T>> for (T, Vec3<T>) {
    fn from(v: Vec4<T>) -> Self {
        (v.x, Vec3::new(v.y, v.z, v.w))
    }
}
