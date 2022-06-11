use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, Hash)]
struct Var {
    sub: isize,
}

#[derive(Debug, Clone, Hash)]
struct Base {
    base: Vec<(Var, isize)>,
}

#[derive(Debug, Clone)]
pub struct Poly {
    terms: HashMap<Base, isize>,
}

impl PartialEq for Base {
    fn eq(&self, rhs: &Self) -> bool {
        // for (var, power) in self.base {}
        true
    }
}
impl Eq for Base {}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (var, coeff) in &self.terms {
            match coeff {
                1 => write!(f, "x"),
                _ => write!(f, "{}x", coeff),
            };
        }
        write!(f, "")
    }
}

impl From<Var> for Poly {
    fn from(var: Var) -> Self {
        let base = Base {
            base: vec![(var.clone(), 1)],
        };
        let terms = HashMap::from([(base, 1)]);
        Poly { terms }
    }
}

impl AddAssign for Poly {
    fn add_assign(&mut self, rhs: Self) {}
}

impl Add for Poly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp += rhs;
        tmp
    }
}

impl Poly {
    fn x() -> Var {
        Var { sub: 0 }
    }
}
