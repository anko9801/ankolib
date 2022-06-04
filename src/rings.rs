pub mod integer_mod_ring;
pub mod ring;

pub trait Ring: Group, ops::Mul {}
