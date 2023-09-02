pub mod algebraic;
mod geometry;
mod graph;
mod util;

use crate::algebraic::{integer::Int, polynomial::FPS};
use num::Integer;

fn main() {
    let f: FPS<isize> =
        FPS::term(5, 3) + ((FPS::term(1, 3) + FPS::term(3, 0)) ^ 2) + FPS::term(3, 1);
    println!("{}", f);
    let a: Int = 15.into();
    let b: Int = 303.into();
    println!("{}", a.gcd(&b));
}
