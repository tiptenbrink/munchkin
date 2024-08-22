use super::ConflictAnalysisContext;
use crate::basic_types::ClauseReference;
use crate::engine::propagation::PropagatorId;
use crate::engine::variables::Literal;
#[cfg(doc)]
use crate::engine::ConstraintSatisfactionSolver;

/// The outcome of clause learning.
#[derive(Clone, Default, Debug)]
pub(crate) struct ConflictAnalysisResult {
    /// The new learned clause with the propagating literal after backjumping at index 0 and the
    /// literal with the next highest decision level at index 1.
    pub(crate) learned_literals: Vec<Literal>,
    /// The decision level to backtrack to.
    pub(crate) backjump_level: usize,
}

#[derive(Default, Debug)]
pub(crate) struct ResolutionConflictAnalyser {
    // TODO
    analysis_result: ConflictAnalysisResult,
}

impl ResolutionConflictAnalyser {
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
    pub(crate) fn compute_1uip(
        &mut self,
        context: &mut ConflictAnalysisContext,
    ) -> ConflictAnalysisResult {
        todo!()
    }

    pub(crate) fn compute_clausal_core(
        &mut self,
        context: &mut ConflictAnalysisContext,
    ) -> Result<Vec<Literal>, Literal> {
        if context.solver_state.is_infeasible() {
            return Ok(vec![]);
        }

        let violated_assumption = context.solver_state.get_violated_assumption();

        // we consider three cases:
        //  1. The assumption is falsified at the root level
        //  2. The assumption is inconsistent with other assumptions, e.g., x and !x given as
        //     assumptions
        //  3. Standard case

        // Case one: the assumption is falsified at the root level
        if context
            .assignments_propositional
            .is_literal_root_assignment(violated_assumption)
        {
            // self.restore_state_at_root(brancher);
            Ok(vec![violated_assumption])
        }
        // Case two: the assumption is inconsistent with other assumptions
        //  i.e., the assumptions contain both literal 'x' and '~x'
        //  not sure what would be the best output in this case, possibly a special flag?
        //      for now we return the reason (x && ~x)
        else if !context
            .assignments_propositional
            .is_literal_propagated(violated_assumption)
        {
            // self.restore_state_at_root(brancher);
            Err(violated_assumption)
        }
        // Case three: the standard case, proceed with core extraction
        // performs resolution on all implied assumptions until only decision assumptions are left
        //  the violating assumption is used as the starting point
        //  at this point, any reason clause encountered will contains only assumptions, but some
        // assumptions might be implied  this corresponds to the all-decision CDCL learning
        // scheme
        else {
            todo!();
            // self.compute_all_decision_learning_helper(
            //     Some(!violated_assumption),
            //     true,
            //     context,
            //     |_| {},
            // );
            self.analysis_result
                .learned_literals
                .push(!violated_assumption);
            // self.restore_state_at_root(brancher);
            Ok(self.analysis_result.learned_literals.clone())
        }
    }

    pub(crate) fn get_conflict_reasons(
        &mut self,
        context: &mut ConflictAnalysisContext,
        on_analysis_step: impl FnMut(AnalysisStep),
    ) {
        let next_literal = if context.solver_state.is_infeasible_under_assumptions() {
            Some(!context.solver_state.get_violated_assumption())
        } else {
            None
        };
        todo!();
        // self.compute_all_decision_learning_helper(next_literal, true, context, on_analysis_step);
    }
}

#[derive(Clone, Debug)]
#[allow(variant_size_differences)]
pub(crate) enum AnalysisStep<'a> {
    AllocatedClause(ClauseReference),
    Propagation {
        propagator: PropagatorId,
        conjunction: &'a [Literal],
        propagated: Literal,
    },
    Unit(Literal),
}
