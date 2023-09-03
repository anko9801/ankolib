use super::{
    ring::{EuclidDomain, Factor, UFD},
    ScalarMul, ScalarPow,
};
use num::{complex::Complex64, traits::NumAssign, BigInt, BigRational, Num, NumCast, PrimInt};
use std::mem;

pub type Int = BigInt;
pub type Rational = BigRational;
pub type Real = f64;
pub type Complex = Complex64;

pub type ZZ = Int;
pub type QQ = Rational;
pub type RR = Real;
pub type CC = Complex;

impl<T: NumAssign + NumCast + Copy> ScalarMul for T {
    fn scalar_mul(&self, e: usize) -> Self {
        self.mul(T::from(e).unwrap())
    }
}
impl<T: NumAssign + PrimInt> ScalarPow for T {
    fn scalar_pow(&self, e: usize) -> Self {
        self.pow(e as u32)
    }
}

pub trait CarmichaelLambda {
    fn carmichael_lambda(self) -> Self;
}

impl<T: PrimInt + NumAssign + UFD> CarmichaelLambda for T {
    fn carmichael_lambda(self) -> Self {
        let n = self;
        let e2 = n.trailing_zeros();
        let mut res = match e2 {
            0 | 1 => 1,
            2 => 2,
            _ => 1 << e2 - 2,
        };
        for (p, e) in (n >> e2 as usize).factors() {
            res = EuclidDomain::lcm(
                res,
                p.scalar_pow((e - 1) as usize)
                    .scalar_mul(p.to_usize().unwrap() - 1)
                    .to_i32()
                    .unwrap(),
            );
        }
        T::from(res).unwrap()
    }
}

impl<T: Num + NumCast> UFD for T {
    fn factors(self) -> Factor<T> {
        Factor {
            i: T::from(2).unwrap(),
            n: self,
        }
    }
}

impl<T: NumAssign + NumCast + PartialOrd + Copy> Iterator for Factor<T> {
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

impl<T: NumAssign + Clone> EuclidDomain for T {
    fn gcd(mut lhs: Self, mut rhs: Self) -> Self {
        while rhs != T::one() {
            let tmp = lhs % rhs.clone();
            lhs = mem::replace(&mut rhs, tmp);
        }
        lhs
    }
    fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self {
        if rhs != T::zero() {
            let d = Self::xgcd(rhs.clone(), lhs.clone() % rhs.clone(), y, x);
            *y -= (lhs.clone() / rhs.clone()) * x.clone();
            d
        } else {
            *x = T::one();
            *y = T::zero();
            lhs
        }
    }

    fn lcm(lhs: Self, rhs: Self) -> Self {
        lhs.clone() / Self::gcd(lhs.clone(), rhs.clone()) * rhs.clone()
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
