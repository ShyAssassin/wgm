use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

// Vec3<T> * T
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Vec3<T> *= T
impl<T: Copy + Mul<Output = T>> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

// Vec3<T> / T
impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// Vec3<T> /= T
impl<T: Copy + Div<Output = T>> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

// Vec3<T> + T
impl<T: Copy + Add<Output = T>> Add<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

// Vec3<T> += T
impl<T: Copy + Add<Output = T>> AddAssign<T> for Vec3<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
    }
}

// Vec3<T> - T
impl<T: Copy + Sub<Output = T>> Sub<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

// Vec3<T> -= T
impl<T: Copy + Sub<Output = T>> SubAssign<T> for Vec3<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
    }
}

// Vec3<T> * Vec3<T>
impl<T: Copy + Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

// Vec3<T> *= Vec3<T>
impl<T: Copy + Mul<Output = T>> MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

// Vec3<T> / Vec3<T>
impl<T: Copy + Div<Output = T>> Div<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

// Vec3<T> /= Vec3<T>
impl<T: Copy + Div<Output = T>> DivAssign<Vec3<T>> for Vec3<T> {
    fn div_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}

// Vec3<T> + Vec3<T>
impl<T: Copy + Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Vec3<T> += Vec3<T>
impl<T: Copy + Add<Output = T>> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

// Vec3<T> - Vec3<T>
impl<T: Copy + Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Vec3<T> -= Vec3<T>
impl<T: Copy + Sub<Output = T>> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}
