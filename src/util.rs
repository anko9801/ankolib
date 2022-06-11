pub use crate::trait_alias;

#[macro_export]
macro_rules! trait_alias {
    ($name:ident = $($t:tt)*) => {
        pub trait $name: $($t)* {}
        impl<T: $($t)*> $name for T {}
    }
}
