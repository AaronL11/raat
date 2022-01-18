use super::structures::{
    algebraic::{Field, Group},
    vectorspaces::VectorSpace,
};
use std::{marker, ops};

/// Trait indicating whether or not our structure has an additive identity.
pub trait AddId: ops::Add + ops::AddAssign + marker::Sized + Copy + Clone {
    /// Returns the additive identity of the structure.
    fn add_id() -> Self;
}

/// Trait indicating whether or not our structure has a multiplicative identity.
pub trait MulId: ops::Mul + ops::MulAssign + marker::Sized + Copy + Clone {
    /// Returns the multiplicative identity of the structure.
    fn mul_id() -> Self;
}

pub trait LinearTransformation<V, U, K, const N: usize, const M: usize>
where
    V: VectorSpace<V, K, N> + Group + ops::Mul<K> + ops::Div<K>,
    U: VectorSpace<U, K, M> + Group + ops::Mul<K> + ops::Div<K>,
    K: Field,
{
}
