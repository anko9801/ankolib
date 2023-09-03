#![allow(non_snake_case)]
pub mod finite_field;
pub mod integer;
pub mod integer_mod;
pub mod matrix;
pub mod minmax;
pub mod polynomial;
pub mod ring;
pub mod tropical;

use crate::util::trait_alias;
use num::{One, Zero};
use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait ScalarMul: Semigroup {
    fn scalar_mul(&self, e: usize) -> Self;
}
pub trait ScalarPow: Semiring {
    fn scalar_pow(&self, e: usize) -> Self;
}

trait_alias! {Semigroup = Add<Output = Self> + AddAssign + Sized}
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
