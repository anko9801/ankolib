use super::{Circle, Line, Point, Real, Segment};

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    // ax + by = c
    pub fn from_coeff(a: Real, b: Real, c: Real) -> Self {
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
    pub fn new(p: Point, r: Real) -> Self {
        Circle { p, r }
    }
}

// // 点の回転方向
// static const int COUNTER_CLOCKWISE = 1;
// static const int CLOCKWISE = -1;
// static const int ONLINE_BACK = 2;
// static const int ONLINE_FRONT = -2;
// static const int ON_SEGMENT = 0;
// int ccw(Point p0, Point p1, Point p2) {
//   Point a = p1 - p0, b = p2 - p0;
//   if (cross(a, b) > EPS) return COUNTER_CLOCKWISE;
//   if (cross(a, b) < -EPS) return CLOCKWISE;
//   if (dot(a, b) < -EPS) return ONLINE_BACK;
//   if (norm(a) < norm(b)) return ONLINE_FRONT;
//   return ON_SEGMENT;
// }

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
