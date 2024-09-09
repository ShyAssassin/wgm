use super::{Deg, Rad};
use num_traits::Float;
use std::f64::consts::PI;
use std::ops::{Mul, Div, Add, Sub};
use std::ops::{MulAssign, DivAssign, AddAssign, SubAssign};

#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Default)]
/// A angle in terms of full rotations.
pub struct Tau<T: Float>(T);

impl<T: Float> Tau<T> {
    /// Create a new `Tau<T>` from the given value.
    pub const fn new(value: T) -> Self {
        return Self(value);
    }

    /// Get the inner value `T` of `Tau<T>`.
    pub const fn inner(&self) -> T {
        return self.0;
    }

    /// Convert a `Deg<T>` to a `Tau<T>`.
    pub fn from_deg(deg: Deg<T>) -> Self {
        return Self::new(deg.inner() / T::from(360).unwrap());
    }

    /// Convert a `Tau<T>` to a `Deg<T>`.
    pub fn to_deg(self) -> Deg<T> {
        return Deg::new(self.inner() * T::from(360).unwrap());
    }

    /// Convert a `Rad<T>` to a `Tau<T>`.
    pub fn from_rad(rad: Rad<T>) -> Self {
        return Self::new(rad.inner() / T::from(2.0 * PI).unwrap());
    }

    /// Convert a `Tau<T>` to a `Rad<T>`.
    pub fn to_rad(self) -> Rad<T> {
        return Rad::new(self.inner() * T::from(2.0 * PI).unwrap());
    }
}

// Tau<T> * Tau<T>
impl<T: Float> Mul<Tau<T>> for Tau<T> {
    type Output = Tau<T>;

    fn mul(self, rhs: Tau<T>) -> Self::Output {
        return Tau::new(self.0 * rhs.0);
    }
}

// Tau<T> *= Tau<T>
impl<T: Float> MulAssign<Tau<T>> for Tau<T> {
    fn mul_assign(&mut self, rhs: Tau<T>) {
        self.0 = self.0 * rhs.0;
    }
}

// Tau<T> / Tau<T>
impl<T: Float> Div<Tau<T>> for Tau<T> {
    type Output = Tau<T>;

    fn div(self, rhs: Tau<T>) -> Self::Output {
        return Tau::new(self.0 / rhs.0);
    }
}

// Tau<T> /= Tau<T>
impl<T: Float> DivAssign<Tau<T>> for Tau<T> {
    fn div_assign(&mut self, rhs: Tau<T>) {
        self.0 = self.0 / rhs.0;
    }
}

// Tau<T> + Tau<T>
impl<T: Float> Add<Tau<T>> for Tau<T> {
    type Output = Tau<T>;

    fn add(self, rhs: Tau<T>) -> Self::Output {
        return Tau::new(self.0 + rhs.0);
    }
}

// Tau<T> += Tau<T>
impl<T: Float> AddAssign<Tau<T>> for Tau<T> {
    fn add_assign(&mut self, rhs: Tau<T>) {
        self.0 = self.0 + rhs.0;
    }
}

// Tau<T> - Tau<T>
impl<T: Float> Sub<Tau<T>> for Tau<T> {
    type Output = Tau<T>;

    fn sub(self, rhs: Tau<T>) -> Self::Output {
        return Tau::new(self.0 - rhs.0);
    }
}

// Tau<T> -= Tau<T>
impl<T: Float> SubAssign<Tau<T>> for Tau<T> {
    fn sub_assign(&mut self, rhs: Tau<T>) {
        self.0 = self.0 - rhs.0;
    }
}
