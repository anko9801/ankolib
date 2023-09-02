use super::{Point, Real};

impl Point {
    pub fn new(x: Real, y: Real) -> Self {
        Self { x, y }
    }
    // 点 p を原点中心に反時計回りに theta 度回転
    pub fn rotate(&self, theta: Real) -> Point {
        Point {
            x: Real::cos(theta) * self.x - Real::sin(theta) * self.y,
            y: Real::sin(theta) * self.x + Real::cos(theta) * self.y,
        }
    }

    pub fn dot(&self, a: Point) -> Point {
        Point {
            x: self.x * a.x,
            y: self.y * a.y,
        }
    }
    pub fn cross(&self, a: Point) -> Real {
        self.x * a.y - self.y * a.x
    }
}

// impl Into<Real> for f64 {
//     fn into(self) -> Real {
//         Real(self)
//     }
// }
// impl Into<f64> for Real {
//     fn into(self) -> f64 {
//         self.0
//     }
// }

// impl PartialEq for Real {
//     fn eq(&self, other: &Self) -> bool {
//         (self.0 - other.0).abs() < EPS
//     }
//     fn ne(&self, other: &Self) -> bool {
//         (self.0 - other.0).abs() >= EPS
//     }
// }

// impl ops::Add<Real> for Real {
//     type Output = Real;

//     fn add(self, _rhs: Real) -> Real {
//         Real(self.0 + _rhs.0)
//     }
// }

// impl ops::Sub<Real> for Real {
//     type Output = Real;

//     fn sub(self, _rhs: Real) -> Real {
//         Real(self.0 - _rhs.0)
//     }
// }

// impl ops::Mul<Real> for Real {
//     type Output = Real;

//     fn mul(self, _rhs: Real) -> Real {
//         Real(self.0 * _rhs.0)
//     }
// }

// impl ops::Div<Real> for Real {
//     type Output = Real;

//     fn div(self, _rhs: Real) -> Real {
//         Real(self.0 / _rhs.0)
//     }
// }

// impl fmt::Display for Real {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

#[test]
fn is() {}
