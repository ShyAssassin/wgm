use core::fmt;
use super::{Deg, Rad};
use num_traits::{Float, FloatConst};
use core::ops::{Add, Sub, Mul, Div, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

/// An angle in turns (tau). One full turn = 1.0 = 2π radians = 360°.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Tau<T>(pub T);

impl<T> Tau<T> {
    /// Create a new `Tau<T>` from the given value.
    pub fn new(value: T) -> Self {
        return Tau(value)
    }

    /// Returns the inner value.
    pub fn inner(self) -> T {
        return self.0
    }

    /// Converts this angle from turns to radians.
    pub fn to_rad(self) -> Rad<T>
    where T: Float + FloatConst {
        let tau = T::TAU();
        return Rad::new(self.0 * tau)
    }

    /// Converts this angle from turns to degrees.
    pub fn to_deg(self) -> Deg<T>
    where T: Float + FloatConst {
        let full = T::from(360.0).unwrap();
        return Deg::new(self.0 * full)
    }
}

impl<T: fmt::Display> fmt::Display for Tau<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}τ", self.0)
    }
}

// Tau + Tau
impl<T: Add<Output = T>> Add for Tau<T> {
    type Output = Tau<T>;
    fn add(self, rhs: Tau<T>) -> Self::Output {
        return Tau(self.0 + rhs.0)
    }
}

// Tau += Tau
impl<T: Add<Output = T> + Copy> AddAssign for Tau<T> {
    fn add_assign(&mut self, rhs: Tau<T>) {
        self.0 = self.0 + rhs.0;
    }
}

// Tau - Tau
impl<T: Sub<Output = T>> Sub for Tau<T> {
    type Output = Tau<T>;
    fn sub(self, rhs: Tau<T>) -> Self::Output {
        return Tau(self.0 - rhs.0)
    }
}

// Tau -= Tau
impl<T: Sub<Output = T> + Copy> SubAssign for Tau<T> {
    fn sub_assign(&mut self, rhs: Tau<T>) {
        self.0 = self.0 - rhs.0;
    }
}

// Tau * T
impl<T: Mul<Output = T>> Mul<T> for Tau<T> {
    type Output = Tau<T>;
    fn mul(self, rhs: T) -> Self::Output {
        return Tau(self.0 * rhs)
    }
}

// Tau *= T
impl<T: Mul<Output = T> + Copy> MulAssign<T> for Tau<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 = self.0 * rhs;
    }
}

// Tau / T
impl<T: Div<Output = T>> Div<T> for Tau<T> {
    type Output = Tau<T>;
    fn div(self, rhs: T) -> Self::Output {
        return Tau(self.0 / rhs)
    }
}

// Tau /= T
impl<T: Div<Output = T> + Copy> DivAssign<T> for Tau<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 = self.0 / rhs;
    }
}

// -Tau
impl<T: Neg<Output = T>> Neg for Tau<T> {
    type Output = Tau<T>;
    fn neg(self) -> Self::Output {
        return Tau(-self.0)
    }
}

// From<Rad<T>> for Tau<T>
impl<T: Float + FloatConst> From<Rad<T>> for Tau<T> {
    fn from(rad: Rad<T>) -> Self {
        let tau = T::TAU();
        return Tau(rad.inner() / tau)
    }
}

// From<Deg<T>> for Tau<T>
impl<T: Float + FloatConst> From<Deg<T>> for Tau<T> {
    fn from(deg: Deg<T>) -> Self {
        let full = T::from(360.0).unwrap();
        return Tau(deg.inner() / full)
    }
}

// From<Tau<T>> for Deg<T>
impl<T: Float + FloatConst> From<Tau<T>> for Deg<T> {
    fn from(tau: Tau<T>) -> Self {
        return tau.to_deg()
    }
}
