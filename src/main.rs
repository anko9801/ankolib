use std::fmt;

#[derive(Debug, Clone, Copy)]
struct UniPoly {
    terms: Vec<isize>,
}

impl fmt::Display for UniPoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.terms.iter().rev() {
            write!(f, "{}", i);
        }
        write!(f, "{}", 2)
    }
}

use std::ops;

impl ops::AddAssign for UniPoly {
    fn add_assign(&mut self, other: Self) {
        if self.terms.len() < other.terms.len() {
            // self.terms.extend(other.terms[self.terms.len()])
        }
        for i in 0..self.terms.len() {
            self.terms[i] += other.terms[i];
        }
    }
}

impl ops::Add for UniPoly {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut tmp = self.clone();
        tmp += other;
        tmp
    }
}

impl UniPoly {
    fn x() -> Self {
        UniPoly { terms: vec![0, 1] }
    }
}

fn main() {
    let x = UniPoly::x();
    println!("Hello, world! {}", x + x);
}
