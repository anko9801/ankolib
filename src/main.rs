pub mod algebraic;
mod geometry;
mod graph;
mod util;

use crate::algebraic::{integer::ZZ, polynomial::FPS};
use num::Integer;

fn main() {
    let a = ZZ::from(0);
    let f: FPS<isize> =
        FPS::term(5, 3) + ((FPS::term(1, 3) + FPS::term(3, 0)) ^ 2) + FPS::term(3, 1);
    println!("{}", f);
    let a: ZZ = 15.into();
    let b: ZZ = 303.into();
    println!("{}", a.gcd(&b));
}
