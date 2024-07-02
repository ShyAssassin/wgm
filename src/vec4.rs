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
