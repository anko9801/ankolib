use crate::algebraic::ring::*;

pub trait CarmichaelLambda {
    fn carmichael_lambda(self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl CarmichaelLambda for $t {
            fn carmichael_lambda(self) -> Self {
                let n = self;
                let e2 = n.trailing_zeros() as $t;
                let mut res = match e2 {
                    0 | 1 => 1,
                    2 => 2,
                    _ => (1 << e2 - 2)
                };
                for (p, e) in (n >> e2).factors() {
                    res = EuclidDomain::lcm(res, p.pow(e-1) * (p-1));
                }
                res
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);
