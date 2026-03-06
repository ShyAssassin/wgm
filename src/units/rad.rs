use core::fmt;
use super::Deg;
use num_traits::{Float, FloatConst};
use core::ops::{Add, Sub, Mul, Div, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

/// An angle in radians.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Rad<T>(pub T);

impl<T> Rad<T> {
    /// Create a new `Rad<T>` from the given value.
    pub fn new(value: T) -> Self {
        return Rad(value)
    }

    /// Returns the inner value.
    pub fn inner(self) -> T {
        return self.0
    }

    /// Converts this angle from radians to degrees.
    pub fn to_deg(self) -> Deg<T>
    where T: Float + FloatConst {
        let factor = T::from(180.0).unwrap() / T::PI();
        return Deg::new(self.0 * factor)
    }
}

impl<T: fmt::Display> fmt::Display for Rad<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} rad", self.0)
    }
}

// Rad + Rad
impl<T: Add<Output = T>> Add for Rad<T> {
    type Output = Rad<T>;
    fn add(self, rhs: Rad<T>) -> Self::Output {
        return Rad(self.0 + rhs.0)
    }
}

// Rad += Rad
impl<T: Add<Output = T> + Copy> AddAssign for Rad<T> {
    fn add_assign(&mut self, rhs: Rad<T>) {
        self.0 = self.0 + rhs.0;
    }
}

// Rad - Rad
impl<T: Sub<Output = T>> Sub for Rad<T> {
    type Output = Rad<T>;
    fn sub(self, rhs: Rad<T>) -> Self::Output {
        return Rad(self.0 - rhs.0)
    }
}

// Rad -= Rad
impl<T: Sub<Output = T> + Copy> SubAssign for Rad<T> {
    fn sub_assign(&mut self, rhs: Rad<T>) {
        self.0 = self.0 - rhs.0;
    }
}

// Rad * T
impl<T: Mul<Output = T>> Mul<T> for Rad<T> {
    type Output = Rad<T>;
    fn mul(self, rhs: T) -> Self::Output {
        return Rad(self.0 * rhs)
    }
}

// Rad *= T
impl<T: Mul<Output = T> + Copy> MulAssign<T> for Rad<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 = self.0 * rhs;
    }
}

// Rad / T
impl<T: Div<Output = T>> Div<T> for Rad<T> {
    type Output = Rad<T>;
    fn div(self, rhs: T) -> Self::Output {
        return Rad(self.0 / rhs)
    }
}

// Rad /= T
impl<T: Div<Output = T> + Copy> DivAssign<T> for Rad<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 = self.0 / rhs;
    }
}

// -Rad
impl<T: Neg<Output = T>> Neg for Rad<T> {
    type Output = Rad<T>;
    fn neg(self) -> Self::Output {
        return Rad(-self.0)
    }
}

// From<Deg<T>> for Rad<T>
impl<T: Float + FloatConst> From<Deg<T>> for Rad<T> {
    fn from(deg: Deg<T>) -> Self {
        return deg.to_rad()
    }
}
