use crate::algebraic::{CommutativeRing, One, ScalarMul, ScalarPow, Zero};
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
        FPS::from(vec![T::zero()])
    }
    fn is_zero(&self) -> bool {
        *self == FPS::from(vec![T::zero()])
    }
}

impl<T: CommutativeRing + Clone + Eq> One for FPS<T> {
    fn one() -> Self {
        FPS::from(vec![T::one()])
    }
}

impl<T: CommutativeRing + Clone + Eq> FPS<T> {
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

    pub fn pseudo_div(&self, other: Self) {}

    pub fn pseudo_rem(&self, other: Self) {}

    // TODO: Half GCD
    pub fn gcd(&self, other: Self) -> Self {
        todo!();
    }

    // pub fn diff(&self) -> Self {
    //     let mut ret = FPS::from(vec![T::zero(); self.degree()]);
    //     for i in 1..=self.degree() {
    //         ret[i - 1] = self[i] * i;
    //     }
    //     ret
    // }

    // pub fn integral(&self) -> Self {
    //     let mut ret = FPS::from(vec![T::zero(); self.degree() + 2]);
    //     for i in 0..=self.degree() {
    //         ret[i + 1] = self[i] / (i + 1);
    //     }
    //     ret
    // }

    fn reduction(&mut self) {
        for i in (0..=self.degree()).rev() {
            if self[i] != T::zero() {
                self.0 = self[..=i].to_vec();
                return;
            }
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> AddAssign for FPS<T> {
    fn add_assign(&mut self, other: Self) {
        if self.degree() < other.degree() {
            self.0.resize(other.degree() + 1, T::zero());
        }
        for i in 0..=other.degree() {
            self[i] += other[i].clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> SubAssign for FPS<T> {
    fn sub_assign(&mut self, other: Self) {
        if self.degree() < other.degree() {
            self.0.resize(other.degree() + 1, T::zero());
        }
        for i in 0..=other.degree() {
            self[i] -= other[i].clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> MulAssign for FPS<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T: CommutativeRing + Clone + Eq> DivAssign for FPS<T> {
    fn div_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if self.degree() < other.degree() {
            *self = FPS::term(T::zero(), 0);
            return;
        }

        // モニック
        let lc = other.leading_coefficient();
        if lc != T::one() {
            return;
        }

        let mut tmp = self.clone();
        *self = FPS::term(T::zero(), 0);
        for power in (0..deg).rev() {
            *self += FPS::term(tmp.leading_coefficient(), power);
            tmp -= FPS::term(tmp.leading_coefficient(), power) * other.clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> RemAssign for FPS<T> {
    fn rem_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if self.degree() < other.degree() {
            *self = FPS::term(T::zero(), 0);
            return;
        }

        // モニック
        let lc = other.leading_coefficient();
        if lc != T::one() {
            return;
        }

        for power in (0..deg).rev() {
            *self -= FPS::term(self.leading_coefficient(), power) * other.clone();
        }
    }
}

impl<T: CommutativeRing + Clone + Eq> BitXorAssign<usize> for FPS<T> {
    fn bitxor_assign(&mut self, other: usize) {
        // let size = self.degree() * other;
        // self.0.resize(size, 0);
        let mut tmp = self.clone();
        let mut bin = 1;
        let mut ans = FPS::term(T::one(), 0);
        while bin <= other {
            if bin & other > 0 {
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
    fn add(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp += other;
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
    fn sub(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= other;
        tmp
    }
}

// TODO: conv multiply
impl<T: CommutativeRing + Clone + Eq> Mul for FPS<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let m = self.degree();
        let n = other.degree();
        let mut coeff = vec![T::zero(); m + n + 1];
        for k in 0..m + n + 1 {
            for i in 0..=k {
                let j = k - i;
                if i > m || j > n {
                    continue;
                }
                coeff[k] += self[i].clone() * other[j].clone();
            }
        }
        FPS::from(coeff)
    }
}

// impl<T: CommutativeRing + Clone + Eq> Mul<usize> for FPS<T> {
//     type Output = Self;
//     fn mul(self, rhs: usize) -> Self {
//         for i in 0..self.degree() {
//             self[i] = self[i] * rhs;
//         }
//         self
//     }
// }

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
    fn div(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp /= other;
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq> Rem for FPS<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp %= other;
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq> BitXor<usize> for FPS<T> {
    type Output = Self;
    fn bitxor(self, other: usize) -> Self {
        let mut tmp = self.clone();
        tmp ^= other;
        tmp
    }
}

impl<T: CommutativeRing + Clone + Eq + Display> Display for FPS<T> {
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
