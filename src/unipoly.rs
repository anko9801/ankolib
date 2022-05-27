use std::fmt;
use std::ops;

#[derive(Debug, Clone)]
pub struct FormalPowerSeries {
    terms: Vec<isize>,
}
type FPS = FormalPowerSeries;

impl fmt::Display for FPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..self.degree()).rev() {
            if i != self.degree() - 1 && self.terms[i] != 0 {
                write!(f, " + ");
            }

            match (self.terms[i], i) {
                (0, _) => (),
                (_, 0) => {
                    write!(f, "{}", self.terms[i]);
                }
                (1, 1) => {
                    write!(f, "x");
                }
                (1, _) => {
                    write!(f, "x^{}", i);
                }
                (_, 1) => {
                    write!(f, "{}x", self.terms[i]);
                }
                _ => {
                    write!(f, "{}x^{}", self.terms[i], i);
                }
            }
        }
        write!(f, "")
    }
}

impl From<isize> for FPS {
    fn from(num: isize) -> Self {
        match num {
            0 => FPS { terms: vec![] },
            _ => FPS { terms: vec![num] },
        }
    }
}

impl ops::AddAssign for FPS {
    fn add_assign(&mut self, other: Self) {
        if self.degree() < other.degree() {
            self.terms.extend(&other.terms[self.degree()..]);
        }
        if self.degree() > other.degree() {
            for i in 0..other.degree() {
                self.terms[i] += other.terms[i];
            }
        } else {
            for i in 0..self.degree() {
                self.terms[i] += other.terms[i];
            }
        }
    }
}

impl ops::Add for FPS {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp += other;
        tmp
    }
}

impl ops::SubAssign for FPS {
    fn sub_assign(&mut self, other: Self) {
        if self.degree() < other.degree() {
            self.terms.extend(&other.terms[self.degree()..]);
        }
        for i in 0..self.degree() {
            self.terms[i] -= other.terms[i];
        }
    }
}

impl ops::Sub for FPS {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= other;
        tmp
    }
}

impl ops::MulAssign for FPS {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl ops::Mul for FPS {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let m = self.degree();
        let n = other.degree();
        let mut coeff = vec![0; m + n - 1];
        for k in 0..m + n - 1 {
            for i in 0..=k {
                let j = k - i;
                if i >= m || j >= n {
                    continue;
                }
                coeff[k] += self.terms[i] * other.terms[j];
            }
        }
        FPS::from_coeff(coeff)
    }
}

impl ops::DivAssign for FPS {
    fn div_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if deg < 0 {
            *self = FPS::integer(0);
            return;
        }

        // モニック
        let lead = other.leading_coefficient();
        if lead != 1 {
            return;
        }

        let mut tmp = self.clone();
        *self = FPS::integer(0);
        for i in (0..deg).rev() {
            *self += FPS::integer(tmp.leading_coefficient()) * (FPS::x() ^ i);
            tmp -= FPS::integer(tmp.leading_coefficient()) * other.clone() * (FPS::x() ^ i);
        }
    }
}

impl ops::Div for FPS {
    type Output = FPS;
    fn div(self, other: Self) -> FPS {
        let mut tmp = self.clone();
        tmp /= other;
        tmp
    }
}

impl ops::RemAssign for FPS {
    fn rem_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if deg < 0 {
            *self = FPS::integer(0);
            return;
        }

        // モニック
        let lead = other.leading_coefficient();
        if lead != 1 {
            return;
        }

        for i in (0..deg).rev() {
            *self -= FPS::integer(self.leading_coefficient()) * other.clone() * (FPS::x() ^ i);
        }
    }
}

impl ops::Rem for FPS {
    type Output = FPS;
    fn rem(self, other: Self) -> FPS {
        let mut tmp = self.clone();
        tmp %= other;
        tmp
    }
}

impl ops::BitXorAssign<usize> for FPS {
    fn bitxor_assign(&mut self, other: usize) {
        // let size = self.degree() * other;
        // self.terms.resize(size, 0);
        let mut tmp = self.clone();
        let mut bin = 1;
        let mut ans = FPS::integer(1);
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

impl ops::BitXor<usize> for FPS {
    type Output = Self;
    fn bitxor(self, other: usize) -> Self {
        let mut tmp = self.clone();
        tmp ^= other;
        tmp
    }
}

impl FPS {
    pub fn gcd(&self, other: Self) -> Self {
        todo!();
    }
}

impl FPS {
    // 不定元 (indeterminate)
    pub fn x() -> FPS {
        Self { terms: vec![0, 1] }
    }

    // 整数 (from)
    pub fn integer(num: isize) -> FPS {
        FPS::from(num)
    }

    // 多項式の係数 (昇冪)
    pub fn coeff(&self) -> Vec<isize> {
        self.terms.clone()
    }

    // 係数
    pub fn from_coeff(coeff: Vec<isize>) -> FPS {
        let mut poly = Self { terms: coeff };
        poly.reduction();
        poly
    }

    fn reduction(&mut self) {
        for i in (0..self.degree()).rev() {
            if self.terms[i] != 0 {
                self.terms = self.terms[..=i].to_vec();
                return;
            }
        }
    }

    // 次数
    pub fn degree(&self) -> usize {
        self.terms.len()
    }

    // 最高次の係数
    pub fn leading_coefficient(&self) -> isize {
        match self.degree() {
            0 => 0,
            _ => self.terms[self.degree() - 1],
        }
    }

    // モニック多項式
    pub fn monic(&mut self) -> Result<FPS, &str> {
        todo!();
    }

    // 代入
    pub fn dubs(&self, x: isize) -> isize {
        todo!();
    }
}
