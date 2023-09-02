mod point;
mod shapes;

use std::fmt;

type Real = f64;
const EPS: Real = 1e-9;
const PI: Real = 3.1415926535897932;

// #[derive(Debug, Clone, Copy)]
pub fn radian_to_degree(r: Real) -> Real {
    r * 180.0 / PI
}
pub fn degree_to_radian(d: Real) -> Real {
    d * PI / 180.0
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: Real,
    y: Real,
}
#[derive(Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    p: Point,
    r: Real,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.a, self.b)
    }
}

// using Points = vector<Point>;
// using Polygon = vector<Point>;
// using Segments = vector<Segment>;
// using Lines = vector<Line>;
// using Circles = vector<Circle>;
