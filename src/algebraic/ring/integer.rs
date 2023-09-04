use super::{EuclidDomain, Factor, UFD};
use crate::algebraic::{ScalarMul, ScalarPow};
use num::{complex::Complex64, traits::NumAssign, BigInt, BigRational, NumCast, PrimInt};
use std::mem;

pub type Int = BigInt;
pub type Rational = BigRational;
pub type Real = f64;
pub type Complex = Complex64;

pub type ZZ = Int;
pub type QQ = Rational;
pub type RR = Real;
pub type CC = Complex;

impl<T: NumAssign + NumCast + Clone> ScalarMul for T {
    fn scalar_mul(&self, e: usize) -> Self {
        self.clone().mul(T::from(e).unwrap())
    }
}
impl<T: NumAssign + NumCast + Clone> ScalarPow for T {
    fn scalar_pow(&self, mut e: usize) -> Self {
        let mut result = T::from(1).unwrap();
        let mut cur = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                result *= cur.clone();
            }
            e >>= 1;
            cur *= cur.clone();
        }
        result
    }
}

pub trait CarmichaelLambda {
    fn carmichael_lambda(self) -> Self;
}

impl<T: PrimInt + NumAssign> CarmichaelLambda for T {
    fn carmichael_lambda(self) -> Self {
        let n = self;
        let e2 = n.trailing_zeros();
        let mut res = match e2 {
            0 | 1 => 1,
            2 => 2,
            _ => 1 << e2 - 2,
        };
        for factor in (n >> e2 as usize).factors() {
            res = EuclidDomain::lcm(
                res,
                factor
                    .p
                    .scalar_pow((factor.e - 1) as usize)
                    .scalar_mul(factor.p.to_usize().unwrap() - 1)
                    .to_i32()
                    .unwrap(),
            );
        }
        T::from(res).unwrap()
    }
}

impl<T: NumAssign + Clone + PartialOrd + PartialEq> UFD for T {
    fn factors(self) -> Vec<Factor<Self>> {
        let mut res = Vec::new();
        let mut n = self.clone();
        if n <= T::one() {
            return res;
        }
        let mut p = T::one() + T::one();

        while p.clone() * p.clone() <= n.clone() {
            while n.clone() % p.clone() == T::zero() {
                let mut e = 1;
                n /= p.clone();
                while n.clone() % p.clone() == T::zero() {
                    n /= p.clone();
                    e += 1;
                }
                res.push(Factor { p: p.clone(), e });
            }
            p += T::one();
        }
        if n != T::one() {
            res.push(Factor { p: n, e: 1 });
        }
        res
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
    let suite: &[(u64, &[Factor<u64>])] = &[
        (0, &[]),
        (1, &[]),
        (2, &[Factor { p: 2, e: 1 }]),
        (3, &[Factor { p: 3, e: 1 }]),
        (4, &[Factor { p: 2, e: 2 }]),
        (5, &[Factor { p: 5, e: 1 }]),
        (10, &[Factor { p: 2, e: 1 }, Factor { p: 5, e: 1 }]),
        (100, &[Factor { p: 2, e: 2 }, Factor { p: 5, e: 2 }]),
        (200, &[Factor { p: 2, e: 3 }, Factor { p: 5, e: 2 }]),
    ];
    for (n, expected) in suite {
        let actual: Vec<_> = n.factors();
        assert_eq!(&actual, expected);
    }
}
