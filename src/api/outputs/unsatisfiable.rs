//! Contains the representation of a unsatisfiable solution.

use crate::branching::Brancher;
use crate::engine::conflict_analysis::ConflictResolver;
use crate::engine::ConstraintSatisfactionSolver;
#[cfg(doc)]
use crate::Solver;

/// A struct which allows the retrieval of an unsatisfiable core consisting of the provided
/// assumptions passed to the initial [`Solver::satisfy_under_assumptions`]. Note that when this
/// struct is dropped (using [`Drop`]) then the [`Solver`] is reset.
#[derive(Debug)]
pub struct UnsatisfiableUnderAssumptions<
    'solver,
    'brancher,
    B: Brancher,
    ConflictResolverType: ConflictResolver,
> {
    pub(crate) solver: &'solver mut ConstraintSatisfactionSolver<ConflictResolverType>,
    pub(crate) brancher: &'brancher mut B,
}

impl<'solver, 'brancher, B: Brancher, ConflictResolverType: ConflictResolver>
    UnsatisfiableUnderAssumptions<'solver, 'brancher, B, ConflictResolverType>
{
    pub fn new(
        solver: &'solver mut ConstraintSatisfactionSolver<ConflictResolverType>,
        brancher: &'brancher mut B,
    ) -> Self {
        UnsatisfiableUnderAssumptions { solver, brancher }
    }
}

impl<'solver, 'brancher, B: Brancher, ConflictResolverType: ConflictResolver> Drop
    for UnsatisfiableUnderAssumptions<'solver, 'brancher, B, ConflictResolverType>
{
    fn drop(&mut self) {
        self.solver.restore_state_at_root(self.brancher)
    }
}
