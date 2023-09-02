pub trait UFD {
    fn factors(self) -> Factor<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Factor<I> {
    pub i: I,
    pub n: I,
}

pub trait EuclidDomain {
    fn gcd(lhs: Self, rhs: Self) -> Self;
    fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self;
    fn lcm(lhs: Self, rhs: Self) -> Self;
}
