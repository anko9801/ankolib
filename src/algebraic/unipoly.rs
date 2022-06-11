use crate::algebraic::{one, zero, CommutativeRing, One, ScalarMul, ScalarPow, Zero};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg,
    RangeToInclusive, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormalPowerSeries<T: CommutativeRing + Clone + Eq>(Vec<T>);

pub type FPS<T> = FormalPowerSeries<T>;

impl<T: CommutativeRing + Clone + Eq> Zero for FPS<T> {
    fn zero() -> Self {
        FPS::from(vec![zero()])
    }
    fn is_zero(&self) -> bool {
        *self == FPS::from(vec![zero()])
    }
}

impl<T: CommutativeRing + Clone + Eq> One for FPS<T> {
    fn one() -> Self {
        FPS::from(vec![one()])
    }
}

impl<T: CommutativeRing + Clone + Eq> FPS<T> {
    // コンストラクタ
    pub fn term(coeff: T, power: usize) -> FPS<T> {
        match (&coeff, power) {
            (_, _) if coeff == zero() => FPS::from(vec![zero()]),
            (_, 0) => FPS::from(vec![coeff]),
            (_, _) => {
                let mut poly = Vec::with_capacity(power);
                for _ in 0..power {
                    poly.push(zero());
                }
                poly.push(coeff);
                FPS::from(poly)
            }
        }
    }

    // 不定元 (indeterminate)
    #[inline]
    pub fn x() -> FPS<T> {
        FPS::term(one(), 1)
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
            0 => zero(),
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

    // pub fn diff(&self) -> Self {
    //     let mut ret = FPS::from(vec![zero(); self.degree()]);
    //     for i in 1..=self.degree() {
    //         ret[i - 1] = self[i] * i;
    //     }
    //     ret
    // }

    // pub fn integral(&self) -> Self {
    //     let mut ret = FPS::from(vec![zero(); self.degree() + 2]);
    //     for i in 0..=self.degree() {
    //         ret[i + 1] = self[i] / (i + 1);
    //     }
    //     ret
    // }

    fn reduction(&mut self) {
        for i in (0..=self.degree()).rev() {
            if self[i] != zero() {
                self.0 = self[..=i].to_vec();
                return;
            }
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> AddAssign for FPS<T> {
    fn add_assign(&mut self, rhs: Self) {
        if self.degree() < rhs.degree() {
            self.0.resize(rhs.degree() + 1, zero());
        }
        for i in 0..=rhs.degree() {
            self[i] += rhs[i].clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> SubAssign for FPS<T> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.degree() < rhs.degree() {
            self.0.resize(rhs.degree() + 1, zero());
        }
        for i in 0..=rhs.degree() {
            self[i] -= rhs[i].clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> MulAssign for FPS<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<T: CommutativeRing + Clone + Eq> DivAssign for FPS<T> {
    fn div_assign(&mut self, rhs: Self) {
        let deg = self.degree() - rhs.degree();
        if self.degree() < rhs.degree() {
            *self = FPS::term(zero(), 0);
            return;
        }

        // モニック
        let lc = rhs.leading_coefficient();
        if lc != one() {
            return;
        }

        let mut tmp = self.clone();
        *self = FPS::term(zero(), 0);
        for power in (0..deg).rev() {
            *self += FPS::term(tmp.leading_coefficient(), power);
            tmp -= FPS::term(tmp.leading_coefficient(), power) * rhs.clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> RemAssign for FPS<T> {
    fn rem_assign(&mut self, rhs: Self) {
        let deg = self.degree() - rhs.degree();
        if self.degree() < rhs.degree() {
            *self = FPS::term(zero(), 0);
            return;
        }

        // モニック
        let lc = rhs.leading_coefficient();
        if lc != one() {
            return;
        }

        for power in (0..deg).rev() {
            *self -= FPS::term(self.leading_coefficient(), power) * rhs.clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> BitXorAssign<usize> for FPS<T> {
    fn bitxor_assign(&mut self, rhs: usize) {
        // let size = self.degree() * rhs;
        // self.0.resize(size, 0);
        let mut tmp = self.clone();
        let mut bin = 1;
        let mut ans = FPS::term(one(), 0);
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

impl<T: CommutativeRing + Clone + Eq> Add for FPS<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp += rhs;
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq> Neg for FPS<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut tmp = self.clone();
        for i in 0..self.degree() {
            tmp[i] = -self[i].clone();
        }
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq> Sub for FPS<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= rhs;
        tmp
    }
}

// TODO: conv multiply
impl<T: CommutativeRing + Clone + Eq> Mul for FPS<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let m = self.degree();
        let n = rhs.degree();
        let mut coeff = vec![zero(); m + n + 1];
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

impl<T: CommutativeRing + Clone + Eq> ScalarMul for FPS<T> {
    fn scalar_mul(&self, rhs: usize) -> Self {
        let mut tmp = self.clone();
        for i in 0..self.degree() {
            tmp[i] = self[i].scalar_mul(rhs);
        }
        tmp
    }
}
impl<T: CommutativeRing + Clone + Eq> ScalarPow for FPS<T> {}

impl<T: CommutativeRing + Clone + Eq> Div for FPS<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp /= rhs;
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq> Rem for FPS<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp %= rhs;
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq> BitXor<usize> for FPS<T> {
    type Output = Self;
    fn bitxor(self, rhs: usize) -> Self {
        let mut tmp = self.clone();
        tmp ^= rhs;
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq + Display> Display for FPS<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in (0..=self.degree()).rev() {
            if i != self.degree() && self[i] != zero() {
                write!(f, " + ")?;
            }

            match (&self[i], i) {
                (_, _) if self[i] == zero() => (),
                (_, 0) => {
                    write!(f, "{}", self[i])?;
                }
                (_, 1) if self[i] == one() => {
                    write!(f, "x")?;
                }
                (_, _) if self[i] == one() => {
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
impl<T: CommutativeRing + Clone + Eq> From<Vec<T>> for FPS<T> {
    fn from(coeff: Vec<T>) -> Self {
        let mut poly = Self { 0: coeff };
        poly.reduction();
        poly
    }
}

impl<T: CommutativeRing + Clone + Eq> Index<usize> for FPS<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: CommutativeRing + Clone + Eq> Index<RangeToInclusive<usize>> for FPS<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: CommutativeRing + Clone + Eq> IndexMut<usize> for FPS<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}
