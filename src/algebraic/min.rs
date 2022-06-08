use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
pub struct Min<T: Ord>(pub T);

impl<T: Ord> Add for Min<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0))
    }
}

impl<T: Ord> AddAssign for Min<T> {
    fn add_assign(&mut self, rhs: Self) {
        if self.0 > rhs.0 {
            *self = rhs;
        }
    }
}

#[derive(Copy, Clone)]
pub struct Max<T: Ord>(pub T);

impl<T: Ord> Add for Max<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.max(rhs.0))
    }
}

impl<T: Ord> AddAssign for Max<T> {
    fn add_assign(&mut self, rhs: Self) {
        if self.0 < rhs.0 {
            *self = rhs;
        }
    }
}
