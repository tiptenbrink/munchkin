use super::Constraint;
use super::NegatableConstraint;
use crate::engine::conflict_analysis::ConflictResolver;
use crate::variables::Literal;
use crate::ConstraintOperationError;
use crate::Solver;

/// Creates the [`NegatableConstraint`] `\/ literal`
///
/// Its negation is `/\ !literal`
pub fn clause(literals: impl Into<Vec<Literal>>) -> impl NegatableConstraint {
    Clause(literals.into())
}

/// Creates the [`NegatableConstraint`] `/\ literal`
///
/// Its negation is `\/ !literal`
pub fn conjunction(literals: impl Into<Vec<Literal>>) -> impl NegatableConstraint {
    Conjunction(literals.into())
}

struct Clause(Vec<Literal>);

impl Constraint for Clause {
    fn post<ConflictResolverType: ConflictResolver>(
        self,
        solver: &mut Solver<ConflictResolverType>,
    ) -> Result<(), ConstraintOperationError> {
        solver.add_clause(self.0)
    }

    fn implied_by<ConflictResolverType: ConflictResolver>(
        self,
        solver: &mut Solver<ConflictResolverType>,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError> {
        solver.add_clause(
            self.0
                .into_iter()
                .chain(std::iter::once(!reification_literal)),
        )
    }
}

impl NegatableConstraint for Clause {
    type NegatedConstraint = Conjunction;

    fn negation(&self) -> Self::NegatedConstraint {
        Conjunction(self.0.iter().map(|&lit| !lit).collect())
    }
}

struct Conjunction(Vec<Literal>);

impl Constraint for Conjunction {
    fn post<ConflictResolverType: ConflictResolver>(
        self,
        solver: &mut Solver<ConflictResolverType>,
    ) -> Result<(), ConstraintOperationError> {
        self.0
            .into_iter()
            .try_for_each(|lit| solver.add_clause([lit]))
    }

    fn implied_by<ConflictResolverType: ConflictResolver>(
        self,
        solver: &mut Solver<ConflictResolverType>,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError> {
        self.0
            .into_iter()
            .try_for_each(|lit| solver.add_clause([!reification_literal, lit]))
    }
}

impl NegatableConstraint for Conjunction {
    type NegatedConstraint = Clause;

    fn negation(&self) -> Self::NegatedConstraint {
        Clause(self.0.iter().map(|&lit| !lit).collect())
    }
}
