use crate::properties;
use std::{marker, ops};

/// Trait for [set](https://en.wikipedia.org/wiki/Set_(mathematics)) like types
pub trait Set: marker::Sized + Default + PartialEq {}

/// Trait for types that form [magmas](https://en.wikipedia.org/wiki/Magma_(algebra))
pub trait Magma: ops::Add<Output = Self> + ops::AddAssign + Set {}

/// Trait for types that form [monoids](https://en.wikipedia.org/wiki/Monoid)
pub trait Monoid: Magma + properties::AddId {}

/// Trait for [group](https://en.wikipedia.org/wiki/Group_(mathematics)) types
pub trait Group: Monoid + ops::Neg + ops::Sub<Output = Self> + ops::SubAssign {}

/// Trait for [rings](https://en.wikipedia.org/wiki/Ring_(mathematics))
pub trait Ring: Group + properties::MulId + ops::Mul<Output = Self> + ops::MulAssign {}

/// Trait for [fields](https://en.wikipedia.org/wiki/Field_(mathematics))
pub trait Field: Ring + ops::Div<Output = Self> + ops::DivAssign {}
