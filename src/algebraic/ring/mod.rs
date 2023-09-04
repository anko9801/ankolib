pub mod finite_field;
pub mod integer;
pub mod integer_mod;
pub mod matrix;
pub mod polynomial;
pub mod tropical;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Factor<T> {
    pub p: T,
    pub e: usize,
}
pub trait UFD {
    fn factors(self) -> Vec<Factor<Self>>
    where
        Self: Sized;
}

pub trait EuclidDomain {
    fn gcd(lhs: Self, rhs: Self) -> Self;
    fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self;
    fn lcm(lhs: Self, rhs: Self) -> Self;
}
