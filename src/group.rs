use std::ops;

pub trait Group: ops::Add, ops::Sub {}

pub trait Monoid {}
