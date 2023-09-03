pub mod algebraic;
mod geometry;
mod graph;
mod util;

use crate::algebraic::{integer::ZZ, polynomial::FPS};
use num::Integer;

fn main() {
    let a = ZZ::from(0);
    let f = FPS::from(vec![3, 0, 0, 1]) ^ 2;
    let f: FPS<isize> = FPS::from(vec![0, 3, 0, 5]) + f;
    println!("{}", f);
    let a: ZZ = 15.into();
    let b: ZZ = 303.into();
    println!("{}", a.gcd(&b));
}
