pub mod field;
pub mod matrix;
pub mod minmax;
pub mod poly;
pub mod ring;
pub mod tropical;
pub mod unipoly;

use crate::util::trait_alias;
use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Zero: Add<Output = Self> + Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
pub trait One: Mul<Output = Self> + Sized {
    fn one() -> Self;
}
pub fn zero<T: Zero>() -> T {
    T::zero()
}
pub fn one<T: One>() -> T {
    T::one()
}

pub trait ScalarMul: Semigroup {
    fn scalar_mul(&self, e: usize) -> Self;
}
pub trait ScalarPow: Semiring {
    fn pow(&self, mut e: usize) -> Self {
        let mut result = Self::one();
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

macro_rules! impl_integer {
    ($($t: ty),*) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    0
                }
                fn is_zero(&self) -> bool {
                    *self == 0
                }
            }
            impl One for $t {
                fn one() -> Self {
                    1
                }
            }

            impl ScalarMul for $t {
                fn scalar_mul(&self, e: usize) -> Self {
                    *self * e as $t
                }
            }
            impl ScalarPow for $t {}
        )*
    };
}
impl_integer! {u64, i64, usize, isize}

trait_alias! {Semigroup = Add<Output = Self> + AddAssign + Sized + Clone}
trait_alias! {Monoid = Semigroup + Zero}
trait_alias! {Group = Monoid + Neg<Output = Self> + Sub<Output = Self> + SubAssign}
trait_alias! {CommutativeSemigroup = Semigroup + ScalarMul}
trait_alias! {CommutativeMonoid = Monoid + ScalarMul}
trait_alias! {Abelian = Group + ScalarMul}

trait_alias! {Semiring = CommutativeMonoid + Mul<Output = Self> + MulAssign + One}
trait_alias! {Ring = Abelian + Mul<Output = Self> + MulAssign + One}
trait_alias! {CommutativeSemiring = Semiring + ScalarPow}
trait_alias! {CommutativeRing = Ring + ScalarPow}
trait_alias! {Field = CommutativeRing + Div<Output = Self> + DivAssign}

// これ演算が被っててよくない <- ほんまか？
pub trait MapMonoid<S: Monoid, F: Monoid> {
    fn mapping(f: &F, x: &S) -> S;
}
