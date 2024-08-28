use crate::basic_types::ConstraintReference;

use super::{ConflictAnalysisContext, ConflictResolver};

pub struct NoLearning;

impl ConflictResolver for NoLearning {
    fn resolve_conflict(&mut self, context: &mut ConflictAnalysisContext) {
        // In the case of no learning, this method does not do anything
    }

    fn process(&mut self, context: &mut ConflictAnalysisContext) -> Result<(), ()> {
        let last_decision = context.get_last_decision();

        context.backtrack(context.get_decision_level() - 1);
        context.enqueue_propagated_literal(!last_decision, ConstraintReference::NULL);
        Ok(())
    }
}
