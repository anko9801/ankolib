pub trait Field: Ring, ops::Div {}

pub trait Fq {
    fn inv(self) -> Self;
    fn square_root(self) -> Self;
}

/*
macro_rules! impl_uint {
    ($t:ty) => {
        impl Fq for $t {
            fn square_root(self) -> Self {
                q
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}
*/
