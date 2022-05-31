extern crate num;
use num::BigInt;
use std::ops;

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

pub struct Rational {
    numerator: Integer,
    denominator: Integer,
}

impl ops::AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        let num = self.numerator.clone() * other.denominator.clone()
            + other.numerator.clone() * self.denominator.clone();
        let den = self.denominator.clone() * other.denominator.clone();
        let g = gcd(&num, &den);
        self.numerator = num / g.clone();
        self.denominator = den / g.clone();
    }
}

impl ops::SubAssign for Rational {
    fn sub_assign(&mut self, other: Self) {
        let num = self.numerator.clone() * other.denominator.clone()
            - other.numerator.clone() * self.denominator.clone();
        let den = self.denominator.clone() * other.denominator.clone();
        let g = gcd(&num, &den);
        self.numerator = num / g.clone();
        self.denominator = den / g;
    }
}

impl ops::MulAssign for Rational {
    fn mul_assign(&mut self, other: Self) {
        self.numerator = self.numerator.clone() * other.numerator;
        self.denominator = self.denominator.clone() * other.denominator;
    }
}

impl ops::DivAssign for Rational {
    fn div_assign(&mut self, other: Self) {
        self.numerator = self.numerator.clone() * other.numerator;
        self.denominator = self.denominator.clone() * other.denominator;
    }
}
