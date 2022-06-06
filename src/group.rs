pub trait Status {
    // a + (b + c) = (a + b) + c
    fn add_associativity() -> Option<bool>;
    // a + b = b + a
    fn add_commutativity() -> Option<bool>;
    // a * (b + c) = a * b + a * c
    fn distributivity() -> Option<bool>;
    // a * (b * c) = (a * b) * c
    fn mul_associativity() -> Option<bool>;
    // a * b = b * a
    fn mul_commutativity() -> Option<bool>;
}

pub trait Monoid {
    fn zero() -> Self;
    fn add(lhs: Self, rhs: Self) -> Self;
}

pub trait Group {
    fn zero() -> Self;
    fn add(lhs: Self, rhs: Self) -> Self;
    fn neg(num: Self) -> Self;
}

// Commutative Monoid + Monoid
pub trait SemiRing {
    fn zero() -> Self;
    fn add(lhs: Self, rhs: Self) -> Self;
    fn one() -> Self;
    fn mul(lhs: Self, rhs: Self) -> Self;
}

// Abelian Group + Monoid
pub trait Ring {
    fn zero() -> Self;
    fn add(lhs: Self, rhs: Self) -> Self;
    fn neg(num: Self) -> Self;
    fn one() -> Self;
    fn mul(lhs: Self, rhs: Self) -> Self;
}

// Abelian Group + Group
pub trait Field {
    fn zero() -> Self;
    fn add(lhs: Self, rhs: Self) -> Self;
    fn neg(num: Self) -> Self;
    fn one() -> Self;
    fn mul(lhs: Self, rhs: Self) -> Self;
    fn inv(num: Self) -> Self;
}

// Fromの実装の仕方を抽象化したい
// 2重マクロでいけそう

// Tropical semi-ring
pub struct MaxPlusSemiRing {
    num: i64,
}
impl SemiRing for MaxPlusSemiRing {
    fn zero() -> Self {
        Self { num: -1 << 60 }
    }
    fn add(lhs: Self, rhs: Self) -> Self {
        Self {
            num: std::cmp::max(lhs.num, rhs.num),
        }
    }
    fn one() -> Self {
        Self { num: 0 }
    }
    fn mul(lhs: Self, rhs: Self) -> Self {
        Self {
            num: lhs.num + rhs.num,
        }
    }
}

pub struct MinPlusSemiRing {
    num: i64,
}
impl SemiRing for MinPlusSemiRing {
    fn zero() -> Self {
        Self { num: 1 << 60 }
    }
    fn add(lhs: Self, rhs: Self) -> Self {
        Self {
            num: std::cmp::max(lhs.num, rhs.num),
        }
    }
    fn one() -> Self {
        Self { num: 0 }
    }
    fn mul(lhs: Self, rhs: Self) -> Self {
        Self {
            num: lhs.num + rhs.num,
        }
    }
}
