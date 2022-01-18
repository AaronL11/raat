use super::algebraic::*;
use std::ops;

/// Traits for types that form [Vector Spaces](https://en.wikipedia.org/wiki/Vector_space)
pub trait VectorSpace<V, K, const N: usize>
where
    V: Group + ops::Mul<K> + ops::Div<K>,
    K: Field,
{
    //
}

/// Traits for types that form [Normed Vector Spaces](https://en.wikipedia.org/wiki/Vector_space)
pub trait NormedVectorSpace<V, K, const N: usize>: VectorSpace<V, K, N>
where
    V: Group + ops::Mul<K> + ops::Div<K>,
    K: Field,
{
    /// [The vector norm]().
    fn norm(&self) -> K;
}

/// Traits for types that form [Inner Product Spaces](https://en.wikipedia.org/wiki/Vector_space)
pub trait InnerProdSpace<V, K, const N: usize>: VectorSpace<V, K, N>
where
    V: Group + ops::Mul<K> + ops::Div<K>,
    K: Field,
{
    /// [The inner product]().
    fn inner_prod(&self, rhs: V) -> K;
}
