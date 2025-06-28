use super::{Rad, Tau};
use num_traits::Float;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A angle in degrees.
pub struct Deg<T: Float>(T);

impl<T: Float> Deg<T> {
    /// Create a new `Deg<T>` from the given value.
    pub const fn new(value: T) -> Self {
        return Self(value);
    }

    /// Get the inner value `T` of `Deg<T>`.
    pub const fn inner(&self) -> T {
        return self.0;
    }

    /// Convert a `Rad<T>` to a `Deg<T>`.
    pub fn from_rad(rad: Rad<T>) -> Self {
        return Self::new(rad.inner().to_degrees());
    }

    /// Convert a `Deg<T>` to a `Rad<T>`.
    pub fn to_rad(self) -> Rad<T> {
        return Rad::new(self.inner().to_radians());
    }

    /// Convert a `Tau<T>` to a `Deg<T>`.
    pub fn from_tau(tau: Tau<T>) -> Self {
        return Self::new(tau.inner() * T::from(360).unwrap());
    }

    /// Convert a `Deg<T>` to a `Tau<T>`.
    pub fn to_tau(self) -> Tau<T> {
        return Tau::new(self.inner() / T::from(360).unwrap());
    }
}

// Deg<T> * Deg<T>
impl<T: Float> Mul<Deg<T>> for Deg<T> {
    type Output = Deg<T>;

    fn mul(self, rhs: Deg<T>) -> Self::Output {
        return Deg::new(self.0 * rhs.0);
    }
}

// Deg<T> *= Deg<T>
impl<T: Float> MulAssign<Deg<T>> for Deg<T> {
    fn mul_assign(&mut self, rhs: Deg<T>) {
        self.0 = self.0 * rhs.0;
    }
}

// Deg<T> / Deg<T>
impl<T: Float> Div<Deg<T>> for Deg<T> {
    type Output = Deg<T>;

    fn div(self, rhs: Deg<T>) -> Self::Output {
        return Deg::new(self.0 / rhs.0);
    }
}

// Deg<T> /= Deg<T>
impl<T: Float> DivAssign<Deg<T>> for Deg<T> {
    fn div_assign(&mut self, rhs: Deg<T>) {
        self.0 = self.0 / rhs.0;
    }
}

// Deg<T> + Deg<T>
impl<T: Float> Add<Deg<T>> for Deg<T> {
    type Output = Deg<T>;

    fn add(self, rhs: Deg<T>) -> Self::Output {
        return Deg::new(self.0 + rhs.0);
    }
}

// Deg<T> += Deg<T>
impl<T: Float> AddAssign<Deg<T>> for Deg<T> {
    fn add_assign(&mut self, rhs: Deg<T>) {
        self.0 = self.0 + rhs.0;
    }
}

// Deg<T> - Deg<T>
impl<T: Float> Sub<Deg<T>> for Deg<T> {
    type Output = Deg<T>;

    fn sub(self, rhs: Deg<T>) -> Self::Output {
        return Deg::new(self.0 - rhs.0);
    }
}

// Deg<T> -= Deg<T>
impl<T: Float> SubAssign<Deg<T>> for Deg<T> {
    fn sub_assign(&mut self, rhs: Deg<T>) {
        self.0 = self.0 - rhs.0;
    }
}
