use super::{
    ring::{EuclidDomain, UFD},
    ScalarMul, ScalarPow,
};
use num::{complex::Complex64, traits::NumAssign, BigInt, BigRational, NumCast, PrimInt};

pub type Int = BigInt;
pub type Rational = BigRational;
pub type Real = f64;
pub type Complex = Complex64;

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
