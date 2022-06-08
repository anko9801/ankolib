use std::fmt;
use std::ops::{
    Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range,
    RangeToInclusive, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Debug, Clone)]
pub struct FormalPowerSeries {
    terms: Vec<isize>,
}
pub type FPS = FormalPowerSeries;

impl FPS {
    // コンストラクタ
    pub fn term(coeff: isize, power: usize) -> FPS {
        match (coeff, power) {
            (0, _) => FPS::from(vec![0]),
            (_, 0) => FPS::from(vec![coeff]),
            (_, _) => {
                let mut poly = Vec::with_capacity(power);
                for _ in 0..power {
                    poly.push(0);
                }
                poly.push(coeff);
                FPS::from(poly)
            }
        }
    }

    // 不定元 (indeterminate)
    #[inline]
    pub fn x() -> FPS {
        FPS::term(1, 1)
    }

    // 多項式の係数 (昇冪)
    #[inline]
    pub fn coeff(&self) -> Vec<isize> {
        self.terms.clone()
    }

    // 次数 (TODO: 0の次数を0ではなく負の無限大とする)
    #[inline]
    pub fn degree(&self) -> usize {
        self.terms.len() - 1
    }

    // 最高次の係数
    pub fn leading_coefficient(&self) -> isize {
        match self.degree() {
            0 => 0,
            deg => self[deg],
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

    pub fn diff(&self) -> Self {
        let mut ret = FPS::from(vec![0; self.degree()]);
        for i in 1..=self.degree() {
            ret[i - 1] = self[i] * i as isize;
        }
        ret
    }

    pub fn integral(&self) -> Self {
        let mut ret = FPS::from(vec![0; self.degree() + 2]);
        for i in 0..=self.degree() {
            ret[i + 1] = self[i] / (i + 1) as isize;
        }
        ret
    }

    fn reduction(&mut self) {
        for i in (0..=self.degree()).rev() {
            if self[i] != 0 {
                self.terms = self[..=i].to_vec();
                return;
            }
        }
    }
}

impl AddAssign for FPS {
    fn add_assign(&mut self, other: Self) {
        if self.degree() < other.degree() {
            self.terms.resize(other.degree() + 1, 0);
        }
        for i in 0..=other.degree() {
            self[i] += other[i];
        }
    }
}

impl SubAssign for FPS {
    fn sub_assign(&mut self, other: Self) {
        if self.degree() < other.degree() {
            self.terms.resize(other.degree() + 1, 0);
        }
        for i in 0..=other.degree() {
            self[i] -= other[i];
        }
    }
}

// TODO: conv multiply
impl Mul for FPS {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let m = self.degree();
        let n = other.degree();
        let mut coeff = vec![0; m + n + 1];
        for k in 0..m + n + 1 {
            for i in 0..=k {
                let j = k - i;
                if i > m || j > n {
                    continue;
                }
                coeff[k] += self[i] * other[j];
            }
        }
        FPS::from(coeff)
    }
}

impl DivAssign for FPS {
    fn div_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if self.degree() < other.degree() {
            *self = FPS::term(0, 0);
            return;
        }

        // モニック
        let lc = other.leading_coefficient();
        if lc != 1 {
            return;
        }

        let mut tmp = self.clone();
        *self = FPS::term(0, 0);
        for power in (0..deg).rev() {
            *self += FPS::term(tmp.leading_coefficient(), power);
            tmp -= FPS::term(tmp.leading_coefficient(), power) * other.clone();
        }
    }
}

impl RemAssign for FPS {
    fn rem_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if self.degree() < other.degree() {
            *self = FPS::term(0, 0);
            return;
        }

        // モニック
        let lc = other.leading_coefficient();
        if lc != 1 {
            return;
        }

        for power in (0..deg).rev() {
            *self -= FPS::term(self.leading_coefficient(), power) * other.clone();
        }
    }
}

impl BitXorAssign<usize> for FPS {
    fn bitxor_assign(&mut self, other: usize) {
        // let size = self.degree() * other;
        // self.terms.resize(size, 0);
        let mut tmp = self.clone();
        let mut bin = 1;
        let mut ans = FPS::term(1, 0);
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

impl Add for FPS {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp += other;
        tmp
    }
}

impl Sub for FPS {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= other;
        tmp
    }
}

impl MulAssign for FPS {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl Div for FPS {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp /= other;
        tmp
    }
}

impl Rem for FPS {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp %= other;
        tmp
    }
}

impl BitXor<usize> for FPS {
    type Output = Self;
    fn bitxor(self, other: usize) -> Self {
        let mut tmp = self.clone();
        tmp ^= other;
        tmp
    }
}

impl fmt::Display for FPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..=self.degree()).rev() {
            if i != self.degree() && self.terms[i] != 0 {
                write!(f, " + ")?;
            }

            match (self.terms[i], i) {
                (0, _) => (),
                (_, 0) => {
                    write!(f, "{}", self.terms[i])?;
                }
                (1, 1) => {
                    write!(f, "x")?;
                }
                (1, _) => {
                    write!(f, "x^{}", i)?;
                }
                (_, 1) => {
                    write!(f, "{}x", self.terms[i])?;
                }
                _ => {
                    write!(f, "{}x^{}", self.terms[i], i)?;
                }
            }
        }
        Ok(())
    }
}

// 係数
impl From<Vec<isize>> for FPS {
    fn from(coeff: Vec<isize>) -> Self {
        let mut poly = Self { terms: coeff };
        poly.reduction();
        poly
    }
}

impl Index<usize> for FPS {
    type Output = isize;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.terms[index]
    }
}

impl Index<RangeToInclusive<usize>> for FPS {
    type Output = [isize];

    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &[isize] {
        &self.terms[index]
    }
}

impl IndexMut<usize> for FPS {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut isize {
        &mut self.terms[index]
    }
}
