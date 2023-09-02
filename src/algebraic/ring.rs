use num::{traits::NumAssign, Num, NumCast};

use crate::util::trait_alias;
use std::mem;

pub trait UniqueFactorizationDomain {
    type Output;
    fn factors(self) -> Self::Output;
}
trait_alias! {UFD = UniqueFactorizationDomain}

#[derive(Debug)]
pub struct FactorsStruct<I> {
    i: I,
    n: I,
}

impl<T: Num + NumCast> UniqueFactorizationDomain for T {
    type Output = FactorsStruct<T>;
    fn factors(self) -> Self::Output {
        Self::Output {
            i: T::from(2).unwrap(),
            n: self,
        }
    }
}
impl<T: NumAssign + NumCast + PartialOrd + Copy> Iterator for FactorsStruct<T> {
    type Item = (T, u32);
    fn next(&mut self) -> Option<(T, u32)> {
        if self.n <= T::one() || self.i == T::zero() {
            return None;
        }
        while self.i * self.i <= self.n {
            while self.n % self.i == T::zero() {
                let mut e = 1;
                self.n /= self.i;
                while self.n % self.i == T::zero() {
                    self.n /= self.i;
                    e += 1;
                }
                return Some((self.i, e));
            }
            self.i += T::one();
        }
        if self.i > T::zero() {
            self.i = T::zero();
            return Some((self.n, 1));
        }
        None
    }
}

pub trait EuclidDomain {
    fn gcd(lhs: Self, rhs: Self) -> Self;
    fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self;
    fn lcm(lhs: Self, rhs: Self) -> Self;
}

impl<T: NumAssign + Copy> EuclidDomain for T {
    fn gcd(mut lhs: Self, mut rhs: Self) -> Self {
        while rhs != T::one() {
            let tmp = lhs % rhs;
            lhs = mem::replace(&mut rhs, tmp);
        }
        lhs
    }
    fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self {
        if rhs != T::zero() {
            let d = Self::xgcd(rhs, lhs % rhs, y, x);
            *y -= (lhs / rhs) * *x;
            d
        } else {
            *x = T::one();
            *y = T::zero();
            lhs
        }
    }

    fn lcm(lhs: Self, rhs: Self) -> Self {
        lhs / Self::gcd(lhs, rhs) * rhs
    }
}

#[test]
fn test_small() {
    let suite: &[(u64, &[(u64, u32)])] = &[
        (0, &[]),
        (1, &[]),
        (2, &[(2, 1)]),
        (3, &[(3, 1)]),
        (4, &[(2, 2)]),
        (5, &[(5, 1)]),
        (10, &[(2, 1), (5, 1)]),
        (100, &[(2, 2), (5, 2)]),
        (200, &[(2, 3), (5, 2)]),
    ];
    for (n, expected) in suite {
        let actual: Vec<_> = n.factors().collect();
        assert_eq!(&actual, expected);
    }
}
