use crate::algebraic::{CommutativeRing, Field, One, ScalarMul, ScalarPow, Zero};
use crate::util::trait_alias;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg,
    RangeToInclusive, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormalPowerSeries<T: CRing>(Vec<T>);
pub type FPS<T> = FormalPowerSeries<T>;
trait_alias! {CRing = CommutativeRing + Clone + Eq + Display}
trait_alias! {Analysis = Field + From<usize> + Clone + Eq + Display}

impl<T: CRing> FPS<T> {
    // コンストラクタ
    pub fn term(coeff: T, power: usize) -> FPS<T> {
        match (&coeff, power) {
            (_, _) if coeff == T::zero() => FPS::from(vec![T::zero()]),
            (_, 0) => FPS::from(vec![coeff]),
            (_, _) => {
                let mut poly = Vec::with_capacity(power);
                for _ in 0..power {
                    poly.push(T::zero());
                }
                poly.push(coeff);
                FPS::from(poly)
            }
        }
    }

    // 不定元 (indeterminate)
    #[inline]
    pub fn x() -> FPS<T> {
        FPS::term(T::one(), 1)
    }

    // 多項式の係数 (昇冪)
    #[inline]
    pub fn coeff(&self) -> Vec<T> {
        self.0.clone()
    }

    // 次数 (TODO: 0の次数を0ではなく負の無限大とする)
    #[inline]
    pub fn degree(&self) -> usize {
        self.0.len() - 1
    }

    // 最高次の係数
    pub fn leading_coefficient(&self) -> T {
        match self.degree() {
            0 => T::zero(),
            deg => self[deg].clone(),
        }
    }

    // モニック多項式
    pub fn monic(&mut self) -> Result<Self, &str> {
        todo!();
    }

    // 代入
    pub fn dubs(&self, x: isize) -> isize {
        todo!();
    }

    pub fn log(&self) {
        todo!();
    }

    pub fn pseudo_div(&self, rhs: Self) {
        todo!();
    }

    pub fn pseudo_rem(&self, rhs: Self) {
        todo!();
    }

    // TODO: Half GCD
    pub fn gcd(&self, rhs: Self) -> Self {
        todo!();
    }

    fn reduction(&mut self) {
        for i in (0..=self.degree()).rev() {
            if self[i] != T::zero() {
                self.0 = self[..=i].to_vec();
                return;
            }
        }
    }
}

impl<T: Analysis> FPS<T> {
    pub fn diff(&self) -> Self {
        let mut ret = FPS::from(vec![T::zero(); self.degree()]);
        for i in 1..=self.degree() {
            ret[i - 1] = self[i].clone() * i.into();
        }
        ret
    }

    pub fn integral(&self) -> Self {
        let mut ret = FPS::from(vec![T::zero(); self.degree() + 2]);
        for i in 0..=self.degree() {
            ret[i + 1] = self[i].clone() / (i + 1).into();
        }
        ret
    }
}

impl<T: CRing> Zero for FPS<T> {
    fn zero() -> Self {
        FPS::from(vec![T::zero()])
    }
    fn is_zero(&self) -> bool {
        *self == FPS::from(vec![T::zero()])
    }
}

impl<T: CRing> One for FPS<T> {
    fn one() -> Self {
        FPS::from(vec![T::one()])
    }
}

impl<T: CRing> AddAssign for FPS<T> {
    fn add_assign(&mut self, rhs: Self) {
        if self.degree() < rhs.degree() {
            self.0.resize(rhs.degree() + 1, T::zero());
        }
        for i in 0..=rhs.degree() {
            self[i] += rhs[i].clone();
        }
    }
}

impl<T: CRing> SubAssign for FPS<T> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.degree() < rhs.degree() {
            self.0.resize(rhs.degree() + 1, T::zero());
        }
        for i in 0..=rhs.degree() {
            self[i] -= rhs[i].clone();
        }
    }
}

impl<T: CRing> MulAssign for FPS<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<T: CRing> DivAssign for FPS<T> {
    fn div_assign(&mut self, rhs: Self) {
        let deg = self.degree() - rhs.degree();
        if self.degree() < rhs.degree() {
            *self = FPS::term(T::zero(), 0);
            return;
        }

        // モニック
        let lc = rhs.leading_coefficient();
        if lc != T::one() {
            return;
        }

        let mut tmp = self.clone();
        *self = FPS::term(T::zero(), 0);
        for power in (0..deg).rev() {
            *self += FPS::term(tmp.leading_coefficient(), power);
            tmp -= FPS::term(tmp.leading_coefficient(), power) * rhs.clone();
        }
    }
}

impl<T: CRing> RemAssign for FPS<T> {
    fn rem_assign(&mut self, rhs: Self) {
        let deg = self.degree() - rhs.degree();
        if self.degree() < rhs.degree() {
            *self = FPS::term(T::zero(), 0);
            return;
        }

        // モニック
        let lc = rhs.leading_coefficient();
        if lc != T::one() {
            return;
        }

        for power in (0..deg).rev() {
            *self -= FPS::term(self.leading_coefficient(), power) * rhs.clone();
        }
    }
}

impl<T: CRing> BitXorAssign<usize> for FPS<T> {
    fn bitxor_assign(&mut self, rhs: usize) {
        // let size = self.degree() * rhs;
        // self.0.resize(size, 0);
        let mut tmp = self.clone();
        let mut bin = 1;
        let mut ans = FPS::term(T::one(), 0);
        while bin <= rhs {
            if bin & rhs > 0 {
                ans *= tmp.clone();
            }
            tmp *= tmp.clone();
            bin <<= 1;
        }
        *self = ans;
    }
}

impl<T: CRing> Add for FPS<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp += rhs;
        tmp
    }
}

impl<T: CRing> Neg for FPS<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut tmp = self.clone();
        for i in 0..self.degree() {
            tmp[i] = -self[i].clone();
        }
        tmp
    }
}

impl<T: CRing> Sub for FPS<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= rhs;
        tmp
    }
}

// TODO: conv multiply
impl<T: CRing> Mul for FPS<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let m = self.degree();
        let n = rhs.degree();
        let mut coeff = vec![T::zero(); m + n + 1];
        for k in 0..m + n + 1 {
            for i in 0..=k {
                let j = k - i;
                if i > m || j > n {
                    continue;
                }
                coeff[k] += self[i].clone() * rhs[j].clone();
            }
        }
        FPS::from(coeff)
    }
}

impl<T: CRing> ScalarMul for FPS<T> {
    fn scalar_mul(&self, rhs: usize) -> Self {
        let mut tmp = self.clone();
        for i in 0..self.degree() {
            tmp[i] = self[i].scalar_mul(rhs);
        }
        tmp
    }
}
impl<T: CRing> ScalarPow for FPS<T> {
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

impl<T: CRing> Div for FPS<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp /= rhs;
        tmp
    }
}

impl<T: CRing> Rem for FPS<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp %= rhs;
        tmp
    }
}

impl<T: CRing> BitXor<usize> for FPS<T> {
    type Output = Self;
    fn bitxor(self, rhs: usize) -> Self {
        let mut tmp = self.clone();
        tmp ^= rhs;
        tmp
    }
}

impl<T: CRing> Display for FPS<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in (0..=self.degree()).rev() {
            if i != self.degree() && self[i] != T::zero() {
                write!(f, " + ")?;
            }

            match (&self[i], i) {
                (_, _) if self[i] == T::zero() => (),
                (_, 0) => {
                    write!(f, "{}", self[i])?;
                }
                (_, 1) if self[i] == T::one() => {
                    write!(f, "x")?;
                }
                (_, _) if self[i] == T::one() => {
                    write!(f, "x^{}", i)?;
                }
                (_, 1) => {
                    write!(f, "{}x", self[i])?;
                }
                _ => {
                    write!(f, "{}x^{}", self[i], i)?;
                }
            }
        }
        Ok(())
    }
}

// 係数
impl<T: CRing> From<Vec<T>> for FPS<T> {
    fn from(coeff: Vec<T>) -> Self {
        let mut poly = Self { 0: coeff };
        poly.reduction();
        poly
    }
}

impl<T: CRing> Index<usize> for FPS<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: CRing> Index<RangeToInclusive<usize>> for FPS<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: CRing> IndexMut<usize> for FPS<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}
