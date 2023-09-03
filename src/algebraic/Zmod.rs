use crate::algebraic::ring::EuclidDomain;
use crate::algebraic::{One, ScalarMul, ScalarPow, Zero};
use num::{BigInt, FromPrimitive, Integer, Num, NumCast};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::integer::ZZ;

pub struct Ideal<T> {
    element: T,
}

#[derive(Debug)]
pub struct IntegerModRing {
    order: ZZ,
}
impl IntegerModRing {
    pub fn new(modulus: ZZ) -> Self {
        // Integer
        if modulus == 0.into() {
            Self { order: modulus }
        } else if modulus < 0.into() {
            Self { order: -modulus }
        } else {
            Self { order: modulus }
        }
    }
    pub fn elem(&self, value: ZZ) -> IntegerMod {
        IntegerMod {
            num: value,
            modulus: self.order.clone(),
        }
    }
    pub fn order(&self) -> ZZ {
        self.order.clone()
    }
    pub fn characteristic(&self) -> ZZ {
        self.order.clone()
    }
    pub fn krull_dimension(&self) -> ZZ {
        0.into()
    }
    pub fn is_noetherian(&self) -> bool {
        true
    }
    pub fn is_prime_field(&self) -> bool {
        todo!();
        // self.0.is_prime()
    }
    pub fn quotient(&self, ideal: Ideal<Self>) -> Self {
        todo!();
    }
    pub fn list_of_elements_of_multiplicative_group(&self) -> Vec<ZZ> {
        // TODO: 32, 64
        let mut multiplicative_group = Vec::new();
        for i in 1..self.order.to_u64_digits().1[0] as usize {
            let num: ZZ = i.into();
            if num.gcd(&self.order) == 1.into() {
                multiplicative_group.push(num);
            }
        }
        multiplicative_group
    }
    // pub fn multiplicative_group_is_cyclic(&self) -> bool {
    //     let mut n = self.order();
    //     if n < 8.into() {
    //         return true;
    //     }

    //     if n % 4 == 0.into() {
    //         return false;
    //     }
    //     if n % 4 == 2.into() {
    //         n = n / ZZ::from(2);
    //     }

    //     true
    //     // return n.is_prime_power();
    // }
    pub fn multiplicative_generator(&self) {}
    pub fn category() {
        // Join of Category of finite commutative rings
        //     and Category of subquotients of monoids
        //     and Category of quotients of semigroups
        //     and Category of finite enumerated sets
    }
}

pub type Zmod = IntegerMod;
#[derive(Clone)]
pub struct IntegerMod {
    num: ZZ,
    modulus: ZZ,
}

// impl<T: Num + NumCast> From<T> for IntegerMod {
//     fn from(value: T) -> Self {
//         Self {
//             num: ZZ::from(value),
//             modulus: 0.into(),
//         }
//     }
// }
impl IntegerMod {
    fn new(num: ZZ, modulus: ZZ) -> Self {
        Self { num, modulus }
    }
    fn value(&self) -> ZZ {
        self.num.clone()
    }
}
impl TryFrom<IntegerMod> for ZZ {
    type Error = ();

    fn try_from(value: IntegerMod) -> Result<Self, Self::Error> {
        if value.modulus == 0.into() {
            Ok(value.num)
        } else {
            Err(())
        }
    }
}

impl Zero for IntegerMod {
    fn zero() -> Self {
        Self {
            num: 0.into(),
            modulus: 0.into(),
        }
    }
    fn is_zero(&self) -> bool {
        self.num == 0.into()
    }
    fn set_zero(&mut self) {
        self.num = 0.into();
    }
}

impl One for IntegerMod {
    fn one() -> Self {
        Self {
            num: 1.into(),
            modulus: 0.into(),
        }
    }
}

impl AddAssign for IntegerMod {
    fn add_assign(&mut self, rhs: Self) {
        assert!(self.modulus == rhs.modulus);
        self.num += rhs.num;
        if self.num >= self.modulus {
            self.num -= self.modulus.clone();
        }
    }
}

impl SubAssign for IntegerMod {
    fn sub_assign(&mut self, rhs: Self) {
        assert!(self.modulus == rhs.modulus);
        self.num -= rhs.num;
        if self.num < 0.into() {
            self.num += self.modulus.clone();
        }
    }
}

impl MulAssign for IntegerMod {
    fn mul_assign(&mut self, rhs: Self) {
        assert!(self.modulus == rhs.modulus);
        self.num *= rhs.num;
        self.num %= self.modulus.clone();
    }
}

impl DivAssign for IntegerMod {
    fn div_assign(&mut self, rhs: Self) {
        assert!(self.modulus == rhs.modulus);
        let (mut x, mut y): (ZZ, ZZ) = (1.into(), 0.into());
        EuclidDomain::xgcd(rhs.num, self.modulus.clone(), &mut x, &mut y);
        self.num = (self.num.clone() * x) % self.modulus.clone();
    }
}

impl Add for IntegerMod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp += rhs;
        tmp
    }
}

impl Neg for IntegerMod {
    type Output = Self;
    fn neg(self) -> Self {
        let mut tmp = self.clone();
        tmp.num = -self.num + self.modulus;
        tmp
    }
}

impl Sub for IntegerMod {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= rhs;
        tmp
    }
}

impl Mul for IntegerMod {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp *= rhs;
        tmp
    }
}

impl Div for IntegerMod {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp /= rhs;
        tmp
    }
}

impl ScalarMul for IntegerMod {
    fn scalar_mul(&self, rhs: usize) -> Self {
        Self {
            num: rhs.into(),
            modulus: self.modulus.clone(),
        } * self.clone()
    }
}

impl ScalarPow for IntegerMod {
    fn scalar_pow(&self, mut e: usize) -> Self {
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

impl Display for IntegerMod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mod {}", self.num, self.modulus)
    }
}

pub mod mod_int {
    use num::{Num, NumCast};

    type ModInternalNum = i64;
    thread_local!(
        static MOD: std::cell::RefCell<ModInternalNum> = std::cell::RefCell::new(0);
    );

    pub fn set_mod_int<T: ToInternalNum>(v: T) {
        MOD.with(|x| x.replace(v.to_internal_num()));
    }
    fn modulo() -> ModInternalNum {
        MOD.with(|x| *x.borrow())
    }

    #[derive(Debug)]
    pub struct ModInt(ModInternalNum);
    impl Clone for ModInt {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }
    impl Copy for ModInt {}

    impl ModInt {
        fn internal_new(mut v: ModInternalNum) -> Self {
            let m = modulo();
            if v >= m {
                v %= m;
            }
            Self(v)
        }

        pub fn internal_pow(&self, mut e: ModInternalNum) -> Self {
            let mut result = 1;
            let mut cur = self.0;
            let modulo = modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= modulo;
                }
                e >>= 1;
                cur = (cur * cur) % modulo;
            }
            Self(result)
        }

        pub fn pow<T>(&self, e: T) -> Self
        where
            T: ToInternalNum,
        {
            self.internal_pow(e.to_internal_num())
        }

        pub fn value(&self) -> ModInternalNum {
            self.0
        }
    }

    pub trait ToInternalNum {
        fn to_internal_num(&self) -> ModInternalNum;
    }
    impl ToInternalNum for ModInt {
        fn to_internal_num(&self) -> ModInternalNum {
            self.0
        }
    }
    impl<T: Num + NumCast> From<T> for ModInt {
        fn from(v: T) -> Self {
            let v = v.to_i64().unwrap();
            Self::internal_new(v)
        }
    }
    impl<T: Num + NumCast> ToInternalNum for T {
        fn to_internal_num(&self) -> ModInternalNum {
            self.to_i64().unwrap()
        }
    }

    impl<T: ToInternalNum> std::ops::AddAssign<T> for ModInt {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }

            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<T: ToInternalNum> std::ops::Add<T> for ModInt {
        type Output = ModInt;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }
    impl<T: ToInternalNum> std::ops::SubAssign<T> for ModInt {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            if rhs > 0 {
                self.0 += m - rhs;
            }
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }
    impl<T: ToInternalNum> std::ops::Sub<T> for ModInt {
        type Output = Self;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }
    impl<T: ToInternalNum> std::ops::MulAssign<T> for ModInt {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }
    impl<T: ToInternalNum> std::ops::Mul<T> for ModInt {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T: ToInternalNum> std::ops::DivAssign<T> for ModInt {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = Self(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<T: ToInternalNum> std::ops::Div<T> for ModInt {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}

#[cfg(test)]
mod test {
    use super::mod_int::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    const PRIME_MOD: [i64; 3] = [1_000_000_007, 1_000_000_009, 998244353];
    const INF: i64 = 1 << 60;

    fn random_add_sub(prime_mod: i64) {
        let mut rng = rand::thread_rng();
        set_mod_int(prime_mod);
        for _ in 0..10000 {
            let x: i64 = rng.sample(Uniform::from(0..prime_mod));
            let y: i64 = rng.sample(Uniform::from(0..prime_mod));

            let mx = ModInt::from(x);
            let my = ModInt::from(y);

            assert_eq!((mx + my).value(), (x + y) % prime_mod);
            assert_eq!((mx + y).value(), (x + y) % prime_mod);
            assert_eq!((mx - my).value(), (x + prime_mod - y) % prime_mod);
            assert_eq!((mx - y).value(), (x + prime_mod - y) % prime_mod);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.value(), x % prime_mod);

            mx += y;
            x += y;
            assert_eq!(mx.value(), x % prime_mod);

            mx -= my;
            x = (x + prime_mod - y % prime_mod) % prime_mod;
            assert_eq!(mx.value(), x);

            mx -= y;
            x = (x + prime_mod - y % prime_mod) % prime_mod;
            assert_eq!(mx.value(), x);
        }
    }

    #[test]
    fn test_random_add_sub1() {
        random_add_sub(PRIME_MOD[0]);
    }

    #[test]
    fn test_random_add_sub2() {
        random_add_sub(PRIME_MOD[1]);
    }

    #[test]
    fn test_random_add_sub3() {
        random_add_sub(PRIME_MOD[2]);
    }

    fn random_mul(prime_mod: i64) {
        let mut rng = rand::thread_rng();
        set_mod_int(prime_mod);
        for _ in 0..10000 {
            let x: i64 = rng.sample(Uniform::from(0..prime_mod));
            let y: i64 = rng.sample(Uniform::from(0..prime_mod));

            let mx = ModInt::from(x);
            let my = ModInt::from(y);

            assert_eq!((mx * my).value(), (x * y) % prime_mod);
            assert_eq!((mx * y).value(), (x * y) % prime_mod);
        }
    }
    #[test]
    fn test_random_mul1() {
        random_mul(PRIME_MOD[0]);
    }
    #[test]
    fn test_random_mul2() {
        random_mul(PRIME_MOD[1]);
    }
    #[test]
    fn test_random_mul3() {
        random_mul(PRIME_MOD[2]);
    }

    #[test]
    fn zero_test() {
        set_mod_int(1_000_000_007i64);
        let a = ModInt::from(1_000_000_000i64);
        let b = ModInt::from(7i64);
        let c = a + b;
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn pow_test() {
        set_mod_int(1_000_000_007i64);
        let a = ModInt::from(3i64);
        let a = a.pow(4i64);
        assert_eq!(a.value(), 81);
    }

    #[test]
    fn div_test() {
        set_mod_int(1_000_000_007i64);
        for i in 1..100000i64 {
            let mut a = ModInt::from(1i64);
            a /= i;
            a *= i;
            assert_eq!(a.value(), 1);
        }
    }

    #[test]
    fn edge_cases() {
        const MOD: i128 = 1_000_000_007;
        set_mod_int(1_000_000_007i64);

        let a = ModInt::from(1_000_000_000i64) * INF;
        assert_eq!(
            a.value(),
            ((1_000_000_000i128 * i128::from(INF)) % MOD) as i64
        );

        let mut a = ModInt::from(1_000_000_000i64);
        a *= INF;
        assert_eq!(
            a.value(),
            ((1_000_000_000i128 * i128::from(INF)) % MOD) as i64
        );

        let a = ModInt::from(1_000_000_000i64) + INF;
        assert_eq!(
            a.value(),
            ((1_000_000_000i128 + i128::from(INF)) % MOD) as i64
        );

        let mut a = ModInt::from(1_000_000_000i64);
        a += INF;
        assert_eq!(
            a.value(),
            ((1_000_000_000i128 + i128::from(INF)) % MOD) as i64
        );

        let a = ModInt::from(1_000_000_000i64) - INF;
        assert_eq!(
            a.value(),
            ((1_000_000_000i128 + MOD - (INF as i128) % MOD) % MOD) as i64
        );

        let mut a = ModInt::from(1_000_000_000i64);
        a -= INF;
        assert_eq!(
            a.value(),
            ((1_000_000_000i128 + MOD - (INF as i128) % MOD) % MOD) as i64
        );

        let a = ModInt::from(1_000_000_000i64) / INF;
        assert_eq!(a.value(), 961239577);

        let mut a = ModInt::from(1_000_000_000i64);
        a /= INF;
        assert_eq!(a.value(), 961239577);
    }

    #[test]
    fn overflow_guard() {
        set_mod_int(1_000_000_007i64);
        let a = ModInt::from(1_000_000_007i64 * 10);
        assert_eq!(a.value(), 0);
    }

    #[test]
    fn initialize_from_various_primitives() {
        set_mod_int(1_000_000_007);
        let a = ModInt::from(100usize);
        let b = ModInt::from(100i64);
        assert_eq!(a.value(), b.value());
    }
}
