use crate::rings::ring::*;

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
                    res = res.lcm(p.pow(e-1) * (p-1));
                }
                res
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr, $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, min!($($rest),+))
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr, $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}
