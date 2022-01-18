//#![warn(missing_docs)]

//! Algebraic Structures
//!
//! This crate creates simple traits that define algebraic structures.

/// The properties those structures have
pub mod properties;
/// The structures themselves
pub mod structures;

macro_rules! implement_trait {
    ($t:ty, $($a:ty),*) => {$(
        impl $a for $t {}
    )*}
}

macro_rules! implement_integers {
    ($($t:ty)*) => {$(
        impl properties::AddId for $t {
            fn add_id() -> $t { 0 }
        }

        impl properties::MulId for $t {
            fn mul_id() -> $t { 1 }
        }
    )*}
}

macro_rules! implement_float {
    ($($t:ty)*) => {$(
        impl properties::AddId for $t {
            fn add_id() -> $t { 0. }
        }

        impl properties::MulId for $t {
            fn mul_id() -> $t { 1. }
        }
    )*}
}

implement_float!(f32 f64);
implement_integers!(u8 u16 u32 u64 u128 usize);
implement_integers!(i8 i16 i32 i64 i128 isize);

macro_rules! implement_monoid {
    ($($t:ty),*) => ($(
            implement_trait!{
            $t,
            structures::algebraic::Set,
            structures::algebraic::Magma,
            structures::algebraic::Monoid
        }
    )*)
}

implement_monoid!(u8, u16, u32, u64, usize);

macro_rules! implement_field {
    ($($t:ty),*) => ($(
            implement_trait!{
            $t,
            structures::algebraic::Set,
            structures::algebraic::Magma,
            structures::algebraic::Monoid,
            structures::algebraic::Group,
            structures::algebraic::Ring,
            structures::algebraic::Field
        }
    )*)
}

implement_field!(i8, i16, i32, i64, isize, f32, f64);
