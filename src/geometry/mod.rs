mod geometric_real;
mod shapes;

use std::fmt;

type Real = f64;
#[derive(Debug, Clone, Copy)]
pub struct GeometricReal(Real);

const EPS: Real = 1e-9;
const PI: Real = 3.1415926535897932;
pub fn radian_to_degree(r: Real) -> Real {
    r * 180.0 / PI
}
pub fn degree_to_radian(d: Real) -> Real {
    d * PI / 180.0
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: GeometricReal,
    y: GeometricReal,
}
#[derive(Debug, Clone, Copy)]
struct Line {
    from: Point,
    to: Point,
}
#[derive(Debug)]
struct Segment {
    from: Point,
    to: Point,
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
        write!(f, "{} to {}", self.from, self.to)
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.from, self.to)
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "from {} radius {}", self.p, self.r)
    }
}

type Points = Vec<Point>;
type Polygon = Vec<Point>;
type Segments = Vec<Segment>;
type Lines = Vec<Line>;
type Circles = Vec<Circle>;
