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
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
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

    pub fn parallel(&self, a: &Line) {}
    // bool parallel(const Line &a, const Line &b) {
    //   return equal(cross(a.b - a.a, b.b - b.a), 0.0);
    // }

    // bool orthogonal(const Line &a, const Line &b) {
    //   return equal(dot(a.b - a.a, b.b - b.a), 0.0);
    // }
}

impl Segment {
    pub fn new(a: Point, b: Point) -> Self {
        Segment(Line { a, b })
    }
}
//   friend istream &operator>>(istream &is, Line &a) { return is >> a.a >> a.b; }

impl Circle {
    pub fn new(p: Point, r: GeometricReal) -> Self {
        Circle { p, r }
    }
}

// // 射影(projection)
// // 直線(線分)lに点pから引いた垂線の足を求める
// Point projection(const Line &l, const Point &p) {
//   Real t = dot(p - l.a, l.a - l.b) / norm(l.a - l.b);
//   return l.a + (l.a - l.b) * t;
// }

// Point projection(const Segment &l, const Point &p) {
//   Real t = dot(p - l.a, l.a - l.b) / norm(l.a - l.b);
//   return l.a + (l.a - l.b) * t;
// }

// // 反射(reflection)
// // 直線lを対称軸として点pと線対称の位置にある点を求める
// Point reflection(const Line &l, const Point &p) {
//   return p + (projection(l, p) - p) * 2.0L;
// }

// // 線分sと線分tが交差しているかどうか
// bool isIntersect(const Segment &s, const Segment &t) {
//   return ccw(s.a, s.b, t.a) * ccw(s.a, s.b, t.b) <= 0 &&
//          ccw(t.a, t.b, s.a) * ccw(t.a, t.b, s.b) <= 0;
// }

// int isIntersect(const Circle &c1, const Circle &c2) {
//   Real d = norm(c1.p - c2.p);
//   // 2つの円が離れている場合
//   if (d > norm(c1.r + c2.r) + EPS) return 4;
//   // 外接している場合
//   if (equal(d, norm(c1.r + c2.r))) return 3;
//   // 内接している場合
//   if (equal(d, norm(c1.r - c2.r))) return 1;
//   // 内包している場合
//   if (d < norm(c1.r - c2.r) - EPS) return 0;
//   // 交差している場合
//   return 2;
// }
