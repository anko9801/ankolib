pub mod field;
pub mod min;
pub mod poly;
pub mod tropical;
// pub mod ring;
pub mod unipoly;

use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Zero: Add<Output = Self> + Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

macro_rules! impl_zero {
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
        )*
    };
}
impl_zero! {u64, i64, usize, isize}

pub trait One: Mul<Output = Self> + Sized {
    fn one() -> Self;
}

pub fn zero<T: Zero>() -> T {
    T::zero()
}

pub fn one<T: One>() -> T {
    T::one()
}

macro_rules! trait_alias {
    ($name:ident = $($t:tt)*) => {
        pub trait $name: $($t)* {}
        impl<T: $($t)*> $name for T {}
    }
}

trait_alias! {Semigroup = Add<Output = Self> + Sized}
trait_alias! {Monoid = Semigroup + Zero}
trait_alias! {CommutativeSemigroup = Semigroup + AddAssign}
trait_alias! {CommutativeMonoid = Monoid + AddAssign}
trait_alias! {Group = Monoid + Neg<Output = Self>}
trait_alias! {Abelian = Group + CommutativeMonoid + Sub<Output = Self> + SubAssign}
trait_alias! {Semiring = CommutativeMonoid + Mul<Output = Self> + Sized + One}
trait_alias! {CommutativeSemiring = Semiring + MulAssign}
trait_alias! {Ring = Semiring + Abelian}
trait_alias! {CommutativeRing = Ring + MulAssign}
trait_alias! {Field = CommutativeRing + Div<Output = Self> + DivAssign}

pub trait MapMonoid<S: Monoid, F: Monoid> {
    fn mapping(f: &F, x: &S) -> S;
}
