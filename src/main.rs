// mod field;
// mod group;
// mod math;
// mod rings;
mod unipoly;

// use crate::math::*;
// use crate::rings::integer_mod_ring::*;
// use crate::rings::ring::*;
use crate::unipoly::FormalPowerSeries;
type FPS = FormalPowerSeries;

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
    let f: FPS = FPS::from(5) * (FPS::x() ^ 3)
        + (((FPS::x() ^ 3) + FPS::from(3)) ^ 2)
        + FPS::from(3) * FPS::x();
    println!("Hello, world! {}", f);
}
