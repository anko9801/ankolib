use num::{complex::Complex64, BigInt, BigRational};

pub type Integer = BigInt;
pub type Rational = BigRational;
pub type Real = f64;
pub type Complex = Complex64;

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
