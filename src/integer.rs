extern crate num;
use num::BigInt;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub type Integer = BigInt;

pub fn gcd(a: &Integer, b: &Integer) -> Integer {
    let mut tmp;
    let mut s = a.clone();
    let mut t = b.clone();
    while t != 0.into() {
        tmp = s % t.clone();
        (s, t) = (t, tmp);
    }
    s
}

// pub fn xgcd(a: &Integer, b: &Integer, x: &mut Integer, y: &mut Integer) -> Integer {
//     let mut d = a.clone();
//     let mut
//     if *b != 0.into() {
//         d = xgcd(b, a % b, y, x);
//         y -= (a / b) * x;
//     } else {
//         *x = 1;
//         *y = 0;
//     }
//     return d;
// }

#[derive(Debug, Clone)]
pub struct Rational {
    numerator: Integer,
    denominator: Integer,
}

impl AddAssign<&Rational> for Rational {
    fn add_assign(&mut self, rhs: &Self) {
        let num = self.numerator.clone() * rhs.denominator.clone()
            + rhs.numerator.clone() * self.denominator.clone();
        let den = self.denominator.clone() * rhs.denominator.clone();
        let g = gcd(&num, &den);
        self.numerator = num / g.clone();
        self.denominator = den / g.clone();
    }
}

impl SubAssign<&Rational> for Rational {
    fn sub_assign(&mut self, rhs: &Self) {
        let num = self.numerator.clone() * rhs.denominator.clone()
            - rhs.numerator.clone() * self.denominator.clone();
        let den = self.denominator.clone() * rhs.denominator.clone();
        let g = gcd(&num, &den);
        self.numerator = num / g.clone();
        self.denominator = den / g.clone();
    }
}

impl MulAssign<&Rational> for Rational {
    fn mul_assign(&mut self, rhs: &Self) {
        self.numerator = self.numerator.clone() * rhs.numerator.clone();
        self.denominator = self.denominator.clone() * rhs.denominator.clone();
    }
}

impl DivAssign<&Rational> for Rational {
    fn div_assign(&mut self, rhs: &Self) {
        self.numerator = self.numerator.clone() * rhs.numerator.clone();
        self.denominator = self.denominator.clone() * rhs.denominator.clone();
    }
}

impl Add<&Rational> for Rational {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp += &rhs;
        tmp
    }
}

impl Sub<&Rational> for Rational {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp -= &rhs;
        tmp
    }
}

impl Mul<&Rational> for Rational {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp *= &rhs;
        tmp
    }
}

impl Div<&Rational> for Rational {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        let mut tmp = self.clone();
        tmp /= &rhs;
        tmp
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}
