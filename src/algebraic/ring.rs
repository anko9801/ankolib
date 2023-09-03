pub trait UFD {
    fn factors(self) -> Vec<(Self, usize)>
    where
        Self: Sized;
}

pub trait EuclidDomain {
    fn gcd(lhs: Self, rhs: Self) -> Self;
    fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self;
    fn lcm(lhs: Self, rhs: Self) -> Self;
}
