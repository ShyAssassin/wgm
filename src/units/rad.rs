use super::{Deg, Tau};
use num_traits::Float;
use std::f64::consts::PI;
use std::ops::{Mul, Div, Add, Sub};
use std::ops::{MulAssign, DivAssign, AddAssign, SubAssign};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A angle in radians.
pub struct Rad<T: Float>(T);

impl<T: Float> Rad<T> {
    /// Create a new `Rad<T>` from the given value.
    pub const fn new(value: T) -> Self {
        return Self(value)
    }

    /// Get the inner value `T` of `Rad<T>`.
    pub const fn inner(&self) -> T {
        return self.0
    }

    /// Convert a `Deg<T>` to a `Rad<T>`.
    pub fn from_deg(deg: Deg<T>) -> Self {
        return Self::new(deg.inner().to_radians())
    }

    /// Convert a `Rad<T>` to a `Deg<T>`.
    pub fn to_deg(self) -> Deg<T> {
        return Deg::new(self.inner().to_degrees());
    }

    ///  Convert a `Tau<T>` to a `Rad<T>`.
    pub fn from_tau(tau: Tau<T>) -> Self {
        return Self::new(tau.inner() * T::from(2.0 * PI).unwrap());
    }

    /// Convert a `Rad<T>` to a `Tau<T>`.
    pub fn to_tau(self) -> Tau<T> {
        return Tau::new(self.inner() / T::from(2.0 * PI).unwrap());
    }
}

// Rad<T> * Rad<T>
impl<T: Float> Mul<Rad<T>> for Rad<T> {
    type Output = Rad<T>;

    fn mul(self, rhs: Rad<T>) -> Self::Output {
        return Rad::new(self.0 * rhs.0);
    }
}

// Rad<T> *= Rad<T>
impl<T: Float> MulAssign<Rad<T>> for Rad<T> {
    fn mul_assign(&mut self, rhs: Rad<T>) {
        self.0 = self.0 * rhs.0;
    }
}

// Rad<T> / Rad<T>
impl<T: Float> Div<Rad<T>> for Rad<T> {
    type Output = Rad<T>;

    fn div(self, rhs: Rad<T>) -> Self::Output {
        return Rad::new(self.0 / rhs.0);
    }
}

// Rad<T> /= Rad<T>
impl<T: Float> DivAssign<Rad<T>> for Rad<T> {
    fn div_assign(&mut self, rhs: Rad<T>) {
        self.0 = self.0 / rhs.0;
    }
}

// Rad<T> + Rad<T>
impl<T: Float> Add<Rad<T>> for Rad<T> {
    type Output = Rad<T>;

    fn add(self, rhs: Rad<T>) -> Self::Output {
        return Rad::new(self.0 + rhs.0);
    }
}

// Rad<T> += Rad<T>
impl<T: Float> AddAssign<Rad<T>> for Rad<T> {
    fn add_assign(&mut self, rhs: Rad<T>) {
        self.0 = self.0 + rhs.0;
    }
}

// Rad<T> - Rad<T>
impl<T: Float> Sub<Rad<T>> for Rad<T> {
    type Output = Rad<T>;

    fn sub(self, rhs: Rad<T>) -> Self::Output {
        return Rad::new(self.0 - rhs.0);
    }
}

// Rad<T> -= Rad<T>
impl<T: Float> SubAssign<Rad<T>> for Rad<T> {
    fn sub_assign(&mut self, rhs: Rad<T>) {
        self.0 = self.0 - rhs.0;
    }
}
