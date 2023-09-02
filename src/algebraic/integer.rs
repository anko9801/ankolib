use super::{ScalarMul, ScalarPow};
use num::{complex::Complex64, traits::NumAssign, BigInt, BigRational, NumCast, PrimInt};

pub type Int = BigInt;
pub type Rational = BigRational;
pub type Real = f64;
pub type Complex = Complex64;

impl<T: NumAssign + NumCast + Copy> ScalarMul for T {
    fn scalar_mul(&self, e: usize) -> Self {
        *self * T::from(e).unwrap()
    }
}
impl<T: NumAssign + NumCast + Copy> ScalarPow for T {
    fn pow(&self, mut e: usize) -> Self {
        let mut result = T::one();
        let mut cur = *self;
        while e > 0 {
            if e & 1 == 1 {
                result *= cur;
            }
            e >>= 1;
            cur *= cur;
        }
        result
    }
}

pub trait CarmichaelLambda {
    fn carmichael_lambda(self) -> Self;
}

// uint
// impl<T: PrimInt + UniqueFactorizationDomain> CarmichaelLambda for T {
//     fn carmichael_lambda(self) -> Self {
//         let n = self;
//         let e2 = n.trailing_zeros();
//         let mut res = match e2 {
//             0 | 1 => 1,
//             2 => 2,
//             _ => (1 << e2 - 2),
//         };
//         for (p, e) in (n >> e2 as usize).factors() {
//             res = EuclidDomain::lcm(res, p.pow(e - 1) * (p - 1));
//         }
//         res
//     }
// }
