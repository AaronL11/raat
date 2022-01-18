use super::algebraic::*;
use std::ops;

/// Traits for [Module](https://en.wikipedia.org/wiki/Module_(mathematics)) like objects.
pub trait Module<V, K>
where
    V: Group + ops::Mul<K>,
    K: Ring,
{
    //
}
