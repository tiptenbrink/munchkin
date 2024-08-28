use log::warn;

use super::Constraint;
use super::NegatableConstraint;
use crate::engine::conflict_analysis::ConflictResolver;
use crate::variables::Literal;
use crate::ConstraintOperationError;
use crate::Solver;

/// A structure which is responsible for adding the created [`Constraint`]s to the
/// [`Solver`]. For an example on how to use this, see [`crate::constraints`].
#[derive(Debug)]
pub struct ConstraintPoster<'solver, ConstraintImpl, ConflictResolverType> {
    solver: &'solver mut Solver<ConflictResolverType>,
    constraint: Option<ConstraintImpl>,
}

impl<'a, ConstraintImpl, ConflictResolverType>
    ConstraintPoster<'a, ConstraintImpl, ConflictResolverType>
{
    pub(crate) fn new(
        solver: &'a mut Solver<ConflictResolverType>,
        constraint: ConstraintImpl,
    ) -> Self {
        ConstraintPoster {
            solver,
            constraint: Some(constraint),
        }
    }
}

impl<ConstraintImpl: Constraint, ConflictResolverType: ConflictResolver>
    ConstraintPoster<'_, ConstraintImpl, ConflictResolverType>
{
    /// Add the [`Constraint`] to the [`Solver`].
    ///
    /// This method returns a [`ConstraintOperationError`] if the addition of the [`Constraint`] led
    /// to a root-level conflict.
    pub fn post(mut self) -> Result<(), ConstraintOperationError> {
        self.constraint.take().unwrap().post(self.solver)
    }

    /// Add the half-reified version of the [`Constraint`] to the [`Solver`]; i.e. post the
    /// constraint `r -> constraint` where `r` is a reification literal.
    ///
    /// This method returns a [`ConstraintOperationError`] if the addition of the [`Constraint`] led
    /// to a root-level conflict.
    pub fn implied_by(
        mut self,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError> {
        self.constraint
            .take()
            .unwrap()
            .implied_by(self.solver, reification_literal)
    }
}

impl<ConstraintImpl: NegatableConstraint, ConflictResolverType: ConflictResolver>
    ConstraintPoster<'_, ConstraintImpl, ConflictResolverType>
{
    /// Add the reified version of the [`Constraint`] to the [`Solver`]; i.e. post the constraint
    /// `r <-> constraint` where `r` is a reification literal.
    ///
    /// This method returns a [`ConstraintOperationError`] if the addition of the [`Constraint`] led
    /// to a root-level conflict.
    pub fn reify(mut self, reification_literal: Literal) -> Result<(), ConstraintOperationError> {
        self.constraint
            .take()
            .unwrap()
            .reify(self.solver, reification_literal)
    }
}

impl<ConstraintImpl, ConflictResolverType> Drop
    for ConstraintPoster<'_, ConstraintImpl, ConflictResolverType>
{
    fn drop(&mut self) {
        if self.constraint.is_some() {
            warn!("A constraint poster is never used, this is likely a mistake.");
        }
    }
}
