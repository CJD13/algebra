use crate::{nonzero::NonZero, operation::O2, set::Subset};

use super::{
    group::Group,
    ring::{Ring, RingOperations},
};

pub trait Field<O>: Ring<O>
where
    O: RingOperations<Self>,
    O::TIMES: O2<NonZero<Self, O>>,
    NonZero<Self, O>: Group<O::TIMES>,
{
    /// Multiplicative inverse. Panics if given zero.
    fn reciprocal(self) -> Self {
        <NonZero<Self, O> as Subset<Self>>::try_from(self)
            .inverse()
            .inclusion()
    }
}
