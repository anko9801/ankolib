use super::{Circle, GeometricReal, Line, Point, Segment};
use std::ops;

const COUNTER_CLOCKWISE: isize = 1;
const CLOCKWISE: isize = -1;
const ONLINE_BACK: isize = 2;
const ONLINE_FRONT: isize = -2;
const ON_SEGMENT: isize = 0;

impl Point {
    pub fn new(x: GeometricReal, y: GeometricReal) -> Self {
        Self { x, y }
    }
    // 点 p を原点中心に反時計回りに theta 度回転
    pub fn rotate(&self, theta: GeometricReal) -> Point {
        Point {
            x: GeometricReal::cos(theta) * self.x - GeometricReal::sin(theta) * self.y,
            y: GeometricReal::sin(theta) * self.x + GeometricReal::cos(theta) * self.y,
        }
    }

    pub fn dot(&self, a: Point) -> GeometricReal {
        self.x * a.x + self.y * a.y
    }
    pub fn cross(&self, a: Point) -> GeometricReal {
        self.x * a.y - self.y * a.x
    }

    pub fn norm(&self) -> GeometricReal {
        self.x * self.x + self.y * self.y
    }
}

// 点の回転方向
pub fn ccw(p0: Point, p1: Point, p2: Point) -> isize {
    let a: Point = p1 - p0;
    let b: Point = p2 - p0;
    if a.cross(b) > 0.0.into() {
        COUNTER_CLOCKWISE
    } else if a.cross(b) < 0.0.into() {
        CLOCKWISE
    } else if a.dot(b) < 0.0.into() {
        ONLINE_BACK
    } else if a.norm() < b.norm() {
        ONLINE_FRONT
    } else {
        ON_SEGMENT
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<GeometricReal> for Point {
    type Output = Point;
    fn mul(self, rhs: GeometricReal) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    // ax + by = c
    pub fn from_coeff(a: GeometricReal, b: GeometricReal, c: GeometricReal) -> Self {
        if a == 0.0.into() {
            Line {
                a: Point::new(0.0.into(), c / b),
                b: Point::new(1.0.into(), c / b),
            }
        } else if b == 0.0.into() {
            Line {
                a: Point::new(c / a, 0.0.into()),
                b: Point::new(c / a, 1.0.into()),
            }
        } else {
            Line {
                a: Point::new(0.0.into(), c / b),
                b: Point::new(c / a, 0.0.into()),
            }
        }
    }

    pub fn parallel(&self, a: Line) -> bool {
        (self.b - self.a).cross(a.b - a.a) == 0.0.into()
    }
    pub fn orthogonal(&self, a: Line) -> bool {
        (self.b - self.a).dot(a.b - a.a) == 0.0.into()
    }

    // 射影(projection)
    // 直線(線分)lに点pから引いた垂線の足を求める
    pub fn projection(&self, p: Point) -> Point {
        let t = (p - self.a).dot(self.a - self.b) / (self.a - self.b).norm();
        self.a + (self.a - self.b) * t
    }

    // 反射(reflection)
    // 直線lを対称軸として点pと線対称の位置にある点を求める
    pub fn reflection(&self, p: Point) -> Point {
        p + (self.projection(p) - p) * 2.0.into()
    }
}

impl Segment {
    pub fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }

    pub fn projection(&self, p: Point) -> Point {
        let t = (p - self.a).dot(self.a - self.b) / (self.a - self.b).norm();
        self.a + (self.a - self.b) * t
    }
    // 線分sと線分tが交差しているかどうか
    pub fn isIntersect(&self, s: Segment) -> bool {
        ccw(s.a, s.b, self.a) * ccw(s.a, s.b, self.b) <= 0
            && ccw(self.a, self.b, s.a) * ccw(self.a, self.b, s.b) <= 0
    }
}
//   friend istream &operator>>(istream &is, Line &a) { return is >> a.a >> a.b; }

impl Circle {
    pub fn new(p: Point, r: GeometricReal) -> Self {
        Circle { p, r }
    }

    pub fn isIntersect(&self, c: Circle) -> usize {
        let d = (self.p - c.p).norm();
        // 2つの円が離れている場合
        if d > (self.r + c.r) {
            4
        // 外接している場合
        } else if d == (self.r + c.r) {
            3
        // 内接している場合
        } else if d == (self.r - c.r) {
            1
        // 内包している場合
        } else if d < (self.r - c.r) {
            0
        // 交差している場合
        } else {
            2
        }
    }
}
