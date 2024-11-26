mod equality;
mod inequality;

pub use equality::*;
pub use inequality::*;

use super::Constraint;
use crate::propagators::arithmetic::maximum::MaximumPropagator;
use crate::variables::IntegerVariable;

/// Creates the [`Constraint`] `a + b = c`.
pub fn plus<Var: IntegerVariable + 'static>(a: Var, b: Var, c: Var) -> impl Constraint {
    equals([a.scaled(1), b.scaled(1), c.scaled(-1)], 0)
}

/// Creates the [`Constraint`] `max(array) = m`.
pub fn maximum<Var: IntegerVariable + 'static>(
    array: impl Into<Box<[Var]>>,
    rhs: impl IntegerVariable + 'static,
) -> impl Constraint {
    MaximumPropagator::new(array.into(), rhs)
}

/// Creates the [`Constraint`] `min(array) = m`.
pub fn minimum<Var: IntegerVariable + 'static>(
    array: impl IntoIterator<Item = Var>,
    rhs: impl IntegerVariable + 'static,
) -> impl Constraint {
    let array = array
        .into_iter()
        .map(|var| var.scaled(-1))
        .collect::<Box<_>>();
    maximum(array, rhs.scaled(-1))
}
