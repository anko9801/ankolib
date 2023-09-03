use super::{
    ring::{EuclidDomain, UFD},
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

impl<T: PrimInt + NumAssign> CarmichaelLambda for T {
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

impl<T: NumAssign + Clone + PartialOrd + PartialEq> UFD for T {
    fn factors(self) -> Vec<(Self, usize)> {
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
                res.push((p.clone(), e));
            }
            p += T::one();
        }
        if p > T::zero() {
            p = T::zero();
            res.push((n, 1));
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
    let suite: &[(u64, &[(u64, usize)])] = &[
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
        let actual: Vec<_> = n.factors();
        assert_eq!(&actual, expected);
    }
}
