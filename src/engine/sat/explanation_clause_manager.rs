use super::ClauseAllocator;
use crate::basic_types::ClauseReference;
use crate::engine::variables::Literal;
use crate::munchkin_assert_moderate;

#[derive(Default, Debug)]
pub(crate) struct ExplanationClauseManager {
    explanation_clauses: Vec<ClauseReference>,
}

impl ExplanationClauseManager {
    #[allow(unused, reason = "can be used in assignment")]
    pub(crate) fn is_empty(&self) -> bool {
        self.explanation_clauses.is_empty()
    }

    pub(crate) fn add_explanation_clause_unchecked(
        &mut self,
        explanation_literals: Vec<Literal>,
        clause_allocator: &mut ClauseAllocator,
    ) -> ClauseReference {
        munchkin_assert_moderate!(explanation_literals.len() >= 2);

        let clause_reference = clause_allocator.create_clause(explanation_literals, false);
        self.explanation_clauses.push(clause_reference);

        clause_reference
    }

    #[allow(unused, reason = "can be used in assignment")]
    pub(crate) fn clean_up_explanation_clauses(&mut self, clause_allocator: &mut ClauseAllocator) {
        // the idea is to delete clauses in reverse order
        //  so that in the future, when we implement manual memory management, we can simply skip
        // large blocks of memory without inspection
        for clause_reference in self.explanation_clauses.iter().rev() {
            clause_allocator.delete_clause(*clause_reference);
        }
        self.explanation_clauses.clear();
    }
}
