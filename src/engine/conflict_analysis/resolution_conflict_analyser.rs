use super::{ConflictAnalysisContext, ConflictResolver};
#[cfg(doc)]
use crate::engine::ConstraintSatisfactionSolver;

#[derive(Default, Debug)]
pub(crate) struct ResolutionConflictAnalyser {
    // TODO
}

impl ConflictResolver for ResolutionConflictAnalyser {
    /// Compute the 1-UIP clause based on the current conflict. According to \[1\] a unit
    /// implication point (UIP), "represents an alternative decision assignment at the current
    /// decision level that results in the same conflict" (i.e. no matter what the variable at the
    /// UIP is assigned, the current conflict will be found again given the current decisions). In
    /// the context of implication graphs used in SAT-solving, a UIP is present at decision
    /// level `d` when the number of literals in the learned clause assigned at decision level
    /// `d` is 1.
    ///
    /// The learned clause which is created by
    /// this method contains a single variable at the current decision level (stored at index 0
    /// of [`ConflictAnalysisResult::learned_literals`]); the variable with the second highest
    /// decision level is stored at index 1 in [`ConflictAnalysisResult::learned_literals`] and its
    /// decision level is (redundantly) stored in [`ConflictAnalysisResult::backjump_level`], which
    /// is used when backtracking in ([`ConstraintSatisfactionSolver`]).
    ///
    /// # Bibliography
    /// \[1\] J. Marques-Silva, I. Lynce, and S. Malik, ‘Conflict-driven clause learning SAT
    /// solvers’, in Handbook of satisfiability, IOS press, 2021
    fn resolve_conflict(&mut self, context: &mut ConflictAnalysisContext) {
        todo!()
    }

    fn process(&mut self, context: &mut ConflictAnalysisContext) -> Result<(), ()> {
        todo!()
    }
}
