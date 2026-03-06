use core::fmt;
use super::Rad;
use num_traits::{Float, FloatConst};
use core::ops::{Add, Sub, Mul, Div, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

/// An angle in degrees.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Deg<T>(pub T);

impl<T> Deg<T> {
    /// Create a new `Deg<T>` from the given value.
    pub fn new(value: T) -> Self {
        return Deg(value)
    }

    /// Returns the inner value.
    pub fn inner(self) -> T {
        return self.0
    }

    /// Converts this angle from degrees to radians.
    pub fn to_rad(self) -> Rad<T>
    where T: Float + FloatConst {
        let factor = T::PI() / T::from(180.0).unwrap();
        return Rad::new(self.0 * factor)
    }
}

impl<T: fmt::Display> fmt::Display for Deg<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}°", self.0)
    }
}

// Deg + Deg
impl<T: Add<Output = T>> Add for Deg<T> {
    type Output = Deg<T>;
    fn add(self, rhs: Deg<T>) -> Self::Output {
        return Deg(self.0 + rhs.0)
    }
}

// Deg += Deg
impl<T: Add<Output = T> + Copy> AddAssign for Deg<T> {
    fn add_assign(&mut self, rhs: Deg<T>) {
        self.0 = self.0 + rhs.0;
    }
}

// Deg - Deg
impl<T: Sub<Output = T>> Sub for Deg<T> {
    type Output = Deg<T>;
    fn sub(self, rhs: Deg<T>) -> Self::Output {
        return Deg(self.0 - rhs.0)
    }
}

// Deg -= Deg
impl<T: Sub<Output = T> + Copy> SubAssign for Deg<T> {
    fn sub_assign(&mut self, rhs: Deg<T>) {
        self.0 = self.0 - rhs.0;
    }
}

// Deg * T
impl<T: Mul<Output = T>> Mul<T> for Deg<T> {
    type Output = Deg<T>;
    fn mul(self, rhs: T) -> Self::Output {
        return Deg(self.0 * rhs)
    }
}

// Deg *= T
impl<T: Mul<Output = T> + Copy> MulAssign<T> for Deg<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 = self.0 * rhs;
    }
}

// Deg / T
impl<T: Div<Output = T>> Div<T> for Deg<T> {
    type Output = Deg<T>;
    fn div(self, rhs: T) -> Self::Output {
        return Deg(self.0 / rhs)
    }
}

// Deg /= T
impl<T: Div<Output = T> + Copy> DivAssign<T> for Deg<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 = self.0 / rhs;
    }
}

// -Deg
impl<T: Neg<Output = T>> Neg for Deg<T> {
    type Output = Deg<T>;
    fn neg(self) -> Self::Output {
        return Deg(-self.0)
    }
}

// From<Rad<T>> for Deg<T>
impl<T: Float + FloatConst> From<Rad<T>> for Deg<T> {
    fn from(rad: Rad<T>) -> Self {
        return rad.to_deg()
    }
}
