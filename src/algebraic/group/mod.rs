pub mod minmax;

use super::ring::integer::ZZ;

trait Group {
    fn is_abelian(&self) -> bool; // AbelianGroup
    fn is_commutative(&self) -> bool {
        self.is_abelian()
    }
    fn order(&self) -> ZZ;
    //     self.cardinality()
    // }
    fn is_finite(&self) -> bool; // Finite Group
                                 //     self.order() != Infinity
                                 // }
    fn is_multiplicative(&self) -> bool {
        true
    }
    fn quotient<G1: Group, G2: Group>(&self, H: G1) -> G2;
}
