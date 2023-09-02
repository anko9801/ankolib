use super::{GeometricReal, Real, EPS};
use std::{fmt, ops};

impl Into<GeometricReal> for Real {
    fn into(self) -> GeometricReal {
        GeometricReal(self)
    }
}
impl Into<Real> for GeometricReal {
    fn into(self) -> Real {
        self.0
    }
}

impl PartialEq for GeometricReal {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPS
    }
    fn ne(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() >= EPS
    }
}
impl PartialOrd for GeometricReal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let diff = self.0 - other.0;
        if diff > EPS {
            Some(std::cmp::Ordering::Greater)
        } else if diff < -EPS {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl ops::Add<GeometricReal> for GeometricReal {
    type Output = GeometricReal;

    fn add(self, _rhs: GeometricReal) -> GeometricReal {
        GeometricReal(self.0 + _rhs.0)
    }
}

impl ops::Sub<GeometricReal> for GeometricReal {
    type Output = GeometricReal;

    fn sub(self, _rhs: GeometricReal) -> GeometricReal {
        GeometricReal(self.0 - _rhs.0)
    }
}

impl ops::Mul<GeometricReal> for GeometricReal {
    type Output = GeometricReal;

    fn mul(self, _rhs: GeometricReal) -> GeometricReal {
        GeometricReal(self.0 * _rhs.0)
    }
}

impl ops::Div<GeometricReal> for GeometricReal {
    type Output = GeometricReal;

    fn div(self, _rhs: GeometricReal) -> GeometricReal {
        GeometricReal(self.0 / _rhs.0)
    }
}

impl fmt::Display for GeometricReal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GeometricReal {
    pub fn sin(self) -> Self {
        GeometricReal(self.0.sin())
    }
    pub fn cos(self) -> Self {
        GeometricReal(self.0.sin())
    }
    pub fn sqrt(self) -> Self {
        GeometricReal(self.0.sqrt())
    }
}
