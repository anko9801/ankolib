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

macro_rules! impl_uint {
    ($t:ty) => {
        impl UniqueFactorizationDomain for $t {
            type Output = FactorsStruct<$t>;
            fn factors(self) -> Self::Output {
                Self::Output { i: 2, n: self }
            }
        }
        impl Iterator for FactorsStruct<$t> {
            type Item = ($t, u32);
            fn next(&mut self) -> Option<($t, u32)> {
                if self.n <= 1 || self.i == 0 {
                    return None;
                }
                while self.i * self.i <= self.n {
                    while self.n % self.i == 0 {
                        let mut e = 1;
                        self.n /= self.i;
                        while self.n % self.i == 0 {
                            self.n /= self.i;
                            e += 1;
                        }
                        return Some((self.i, e));
                    }
                    self.i += 1;
                }
                if self.i > 0 {
                    self.i = 0;
                    return Some((self.n, 1));
                }
                None
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

pub trait EuclidDomain {
    fn gcd(lhs: Self, rhs: Self) -> Self;
    fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self;
    fn lcm(lhs: Self, rhs: Self) -> Self;
}

macro_rules! impl_integer {
    ($t:ty) => {
        impl EuclidDomain for $t {
            fn gcd(mut lhs: Self, mut rhs: Self) -> Self {
                while rhs != 0 {
                    let tmp = lhs % rhs;
                    lhs = mem::replace(&mut rhs, tmp);
                }
                lhs
            }
            fn xgcd(lhs: Self, rhs: Self, x: &mut Self, y: &mut Self) -> Self {
                if rhs != 0 {
                    let d = Self::xgcd(rhs, lhs % rhs, y, x);
                    *y -= (lhs / rhs) * *x;
                    d
                }else{
                    *x = 1;
                    *y = 0;
                    lhs
                }
            }

            fn lcm(lhs: Self, rhs: Self) -> Self {
                lhs / Self::gcd(lhs, rhs) * rhs
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_integer!($t);)* };
}
impl_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

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
