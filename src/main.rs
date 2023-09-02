pub mod algebraic;
mod geometry;
mod graph;
mod util;

use num::Integer;

// use crate::math::*;
// use crate::rings::integer_mod_ring::*;
// use crate::rings::ring::*;
use crate::algebraic::{integer::Int, unipoly::FPS};

fn main() {
    // let R = Zmod::from(20);
    // let a = R.elem(3);
    // println!("{}", a);
    // a += 3;
    // println!("{}", a);
    // println!("Hello, world!");
    // println!("{}", -18 % 12);
    // println!("{}", 30_i64.gcd(-18));
    // println!("{}", 30.lcm(-18));
    // println!("{:?}", 735134400_u64.factors().collect::<Vec<_>>());
    // println!("{}", 30_u32.carmichael_lambda());
    let f: FPS<isize> =
        FPS::term(5, 3) + ((FPS::term(1, 3) + FPS::term(3, 0)) ^ 2) + FPS::term(3, 1);
    println!("{}", f);
    let a: Int = 15.into();
    let b: Int = 303.into();
    println!("{}", a.gcd(&b));
}
