use std::fmt;
use std::ops;

#[derive(Debug, Clone)]
pub struct FormalPowerSeries {
    terms: Vec<isize>,
}

impl fmt::Display for FormalPowerSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.degree() {
            match (self.terms[i], i) {
                (0, _) => (),
                (_, 0) => {
                    write!(f, " + {}", self.terms[i]);
                }
                (1, 1) => {
                    write!(f, " + x");
                }
                (1, _) => {
                    write!(f, " + x^{}", i);
                }
                (_, 1) => {
                    write!(f, " + {}x", self.terms[i]);
                }
                _ => {
                    write!(f, " + {}x^{}", self.terms[i], i);
                }
            }
        }
        write!(f, "")
    }
}

impl From<isize> for FormalPowerSeries {
    fn from(num: isize) -> Self {
        match num {
            0 => FormalPowerSeries { terms: vec![] },
            _ => FormalPowerSeries { terms: vec![num] },
        }
    }
}

impl ops::AddAssign for FormalPowerSeries {
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

impl ops::Add for FormalPowerSeries {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp += other;
        tmp
    }
}

impl ops::SubAssign for FormalPowerSeries {
    fn sub_assign(&mut self, other: Self) {
        if self.degree() < other.degree() {
            self.terms.extend(&other.terms[self.degree()..]);
        }
        for i in 0..self.degree() {
            self.terms[i] -= other.terms[i];
        }
    }
}

impl ops::Sub for FormalPowerSeries {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= other;
        tmp
    }
}

impl ops::MulAssign for FormalPowerSeries {
    fn mul_assign(&mut self, other: Self) {
        let m = self.degree();
        let n = other.degree();
        let mut coeff = vec![0; m + n + 1];
        for k in 0..=m + n {
            for i in 0..=k {
                let j = k - i;
                if i >= m || j >= n {
                    continue;
                }
                coeff[k] += self.terms[i] * other.terms[j];
            }
        }
        self.terms = coeff;
    }
}

impl ops::Mul for FormalPowerSeries {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let m = self.degree();
        let n = other.degree();
        let mut coeff = vec![0; m + n + 1];
        for k in 0..=m + n {
            for i in 0..=k {
                let j = k - i;
                if i >= m || j >= n {
                    continue;
                }
                coeff[k] += self.terms[i] * other.terms[j];
            }
        }
        FormalPowerSeries::from_coeff(coeff)
    }
}

impl ops::DivAssign for FormalPowerSeries {
    fn div_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if deg < 0 {
            *self = FormalPowerSeries::integer(0);
            return;
        }

        // モニック
        let lead = other.leading_coefficient();
        if lead != 1 {
            return;
        }

        let mut tmp = self.clone();
        *self = FormalPowerSeries::integer(0);
        for i in (0..deg).rev() {
            *self += FormalPowerSeries::integer(tmp.leading_coefficient())
                * (FormalPowerSeries::x() ^ i);
            tmp -= FormalPowerSeries::integer(tmp.leading_coefficient())
                * other.clone()
                * (FormalPowerSeries::x() ^ i);
        }
    }
}

impl ops::Div for FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn div(self, other: Self) -> FormalPowerSeries {
        let mut tmp = self.clone();
        tmp /= other;
        tmp
    }
}

impl ops::RemAssign for FormalPowerSeries {
    fn rem_assign(&mut self, other: Self) {
        let deg = self.degree() - other.degree();
        if deg < 0 {
            *self = FormalPowerSeries::integer(0);
            return;
        }

        // モニック
        let lead = other.leading_coefficient();
        if lead != 1 {
            return;
        }

        for i in (0..deg).rev() {
            *self -= FormalPowerSeries::integer(self.leading_coefficient())
                * other.clone()
                * (FormalPowerSeries::x() ^ i);
        }
    }
}

impl ops::Rem for FormalPowerSeries {
    type Output = FormalPowerSeries;
    fn rem(self, other: Self) -> FormalPowerSeries {
        let mut tmp = self.clone();
        tmp %= other;
        tmp
    }
}

impl ops::BitXorAssign<usize> for FormalPowerSeries {
    fn bitxor_assign(&mut self, other: usize) {
        // let size = self.degree() * other;
        // self.terms.resize(size, 0);
        let mut tmp = self.clone();
        let mut bin = 1;
        let mut ans = FormalPowerSeries::integer(1);
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

impl ops::BitXor<usize> for FormalPowerSeries {
    type Output = Self;
    fn bitxor(self, other: usize) -> Self {
        let mut tmp = self.clone();
        tmp ^= other;
        tmp
    }
}

impl FormalPowerSeries {
    pub fn gcd(&self, other: Self) -> Self {
        todo!();
    }
}

impl FormalPowerSeries {
    // 不定元 (indeterminate)
    pub fn x() -> FormalPowerSeries {
        Self { terms: vec![0, 1] }
    }

    // 整数 (from)
    pub fn integer(num: isize) -> FormalPowerSeries {
        FormalPowerSeries::from(num)
    }

    // 多項式の係数 (昇冪)
    pub fn coeff(&self) -> Vec<isize> {
        self.terms.clone()
    }

    // 係数
    pub fn from_coeff(coeff: Vec<isize>) -> FormalPowerSeries {
        let mut poly = Self { terms: coeff };
        poly.reduction();
        poly
    }

    fn reduction(&mut self) {
        for i in (0..self.degree()).rev() {
            if self.terms[i] != 0 {
                self.terms = self.terms[..=i].to_vec();
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
    pub fn monic(&mut self) -> Result<FormalPowerSeries, &str> {
        todo!();
    }

    // 代入
    pub fn dubs(&self, x: isize) -> isize {
        todo!();
    }
}
