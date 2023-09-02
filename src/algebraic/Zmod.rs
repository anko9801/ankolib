use num::{Num, NumCast};

use crate::algebraic::ring::EuclidDomain;
use crate::algebraic::{One, ScalarMul, ScalarPow, Zero};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

type ZmodInt = i64;

#[derive(Copy, Clone)]
pub struct Zmod {
    num: ZmodInt,
    MOD: ZmodInt,
}

impl<T: Num + NumCast> From<T> for Zmod {
    fn from(value: T) -> Self {
        Self {
            num: value.to_i64().unwrap(),
            MOD: 0,
        }
    }
}
impl Zmod {
    fn new(num: ZmodInt, MOD: ZmodInt) -> Self {
        Self { num, MOD }
    }
    fn modint(MOD: ZmodInt) -> Self {
        Self { num: 0, MOD }
    }
    fn num(&self, num: ZmodInt) -> Self {
        Self { num, MOD: self.MOD }
    }
    fn value(&self) -> ZmodInt {
        self.num
    }

    fn check_mod(&self, rhs: Self) -> ZmodInt {
        match (self.MOD, rhs.MOD) {
            (0, 0) => ZmodInt::MAX,
            (0, _) => rhs.MOD,
            (_, 0) => self.MOD,
            (_, _) => {
                if self.MOD == rhs.MOD {
                    self.MOD
                } else {
                    panic!("MOD is not matched")
                }
            }
        }
    }
}

impl Zero for Zmod {
    fn zero() -> Self {
        Self { num: 0, MOD: 0 }
    }
    fn is_zero(&self) -> bool {
        self.num == 0
    }
}

impl One for Zmod {
    fn one() -> Self {
        Self { num: 1, MOD: 0 }
    }
}

impl AddAssign for Zmod {
    fn add_assign(&mut self, rhs: Self) {
        let pmod = self.check_mod(rhs);
        self.num += rhs.num;
        if self.num >= pmod {
            self.num -= pmod;
        }
    }
}

impl SubAssign for Zmod {
    fn sub_assign(&mut self, rhs: Self) {
        let pmod = self.check_mod(rhs);
        self.num -= rhs.num;
        if self.num < 0 {
            self.num += pmod;
        }
    }
}

impl MulAssign for Zmod {
    fn mul_assign(&mut self, rhs: Self) {
        let pmod = self.check_mod(rhs);
        self.num *= rhs.num;
        self.num %= pmod;
    }
}

impl DivAssign for Zmod {
    fn div_assign(&mut self, rhs: Self) {
        let pmod = self.check_mod(rhs);
        let (mut x, mut y) = (1, 0);
        EuclidDomain::xgcd(rhs.num, pmod, &mut x, &mut y);
        self.num = (self.num * x) % pmod;
    }
}

impl Add for Zmod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp += rhs;
        tmp
    }
}

impl Neg for Zmod {
    type Output = Self;
    fn neg(self) -> Self {
        let mut tmp = self.clone();
        tmp.num = -self.num + self.MOD;
        tmp
    }
}

impl Sub for Zmod {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= rhs;
        tmp
    }
}

impl Mul for Zmod {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp *= rhs;
        tmp
    }
}

impl Div for Zmod {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp /= rhs;
        tmp
    }
}

impl ScalarMul for Zmod {
    fn scalar_mul(&self, rhs: usize) -> Self {
        *self * rhs.into()
    }
}

impl ScalarPow for Zmod {
    fn scalar_pow(&self, mut e: usize) -> Self {
        let mut result = Self::one();
        let mut cur = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                result *= cur;
            }
            e >>= 1;
            cur *= cur;
        }
        result
    }
}

impl Display for Zmod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mod {}", self.num, self.MOD)
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
