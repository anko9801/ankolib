mod point;
mod shapes;

use std::fmt;

type Real = f64;
#[derive(Debug, Clone, Copy)]
pub struct GeometricReal(Real);

const EPS: Real = 1e-9;
const PI: Real = 3.1415926535897932;

// pub fn radian_to_degree(r: Real) -> Real {
//     r * 180.0 / PI
// }
// pub fn degree_to_radian(d: Real) -> Real {
//     d * PI / 180.0
// }

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: GeometricReal,
    y: GeometricReal,
}
#[derive(Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}
#[derive(Debug)]
struct Segment {
    a: Point,
    b: Point,
}
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    p: Point,
    r: GeometricReal,
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

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "from {} radius {}", self.p, self.r)
    }
}

// using Points = vector<Point>;
// using Polygon = vector<Point>;
// using Segments = vector<Segment>;
// using Lines = vector<Line>;
// using Circles = vector<Circle>;
