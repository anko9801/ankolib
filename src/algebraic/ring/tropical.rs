use crate::algebraic::{One, Zero};
use std::ops::{Add, AddAssign, Mul};

// Tropical semi-ring
#[derive(Clone, Copy)]
pub struct MaxPlusSemiring(i64);

impl Zero for MaxPlusSemiring {
    fn zero() -> Self {
        Self(-1 << 60)
    }
    fn is_zero(&self) -> bool {
        self.0 <= -1 << 60
    }
}

impl Add for MaxPlusSemiring {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(std::cmp::max(self.0, rhs.0))
    }
}

impl AddAssign for MaxPlusSemiring {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl One for MaxPlusSemiring {
    fn one() -> Self {
        Self(0)
    }
}

impl Mul for MaxPlusSemiring {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

#[derive(Clone, Copy)]
pub struct MinPlusSemiring(i64);

impl Zero for MinPlusSemiring {
    fn zero() -> Self {
        Self(1 << 60)
    }
    fn is_zero(&self) -> bool {
        self.0 >= 1 << 60
    }
}

impl Add for MinPlusSemiring {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(std::cmp::min(self.0, rhs.0))
    }
}

impl AddAssign for MinPlusSemiring {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl One for MinPlusSemiring {
    fn one() -> Self {
        Self(0)
    }
}

impl Mul for MinPlusSemiring {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
