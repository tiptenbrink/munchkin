use super::AssignmentsPropositional;
use crate::variables::Literal;

/// Given a clause (consisting of [`Literal`]s), this method should calculate the Literal Block
/// Distance
pub(crate) fn calculate_lbd(_clause: &[Literal], _assignments: &AssignmentsPropositional) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::engine::sat::calculate_lbd;
    use crate::engine::test_helper::TestSolver;

    #[test]
    fn simple_compute_lbd() {
        let mut solver = TestSolver::default();

        let x = solver.new_literal();
        let y = solver.new_literal();
        let z = solver.new_literal();
        let a = solver.new_literal();

        solver.assignments_propositional.increase_decision_level();
        solver.assignments_propositional.enqueue_decision_literal(x);

        solver.assignments_propositional.increase_decision_level();
        solver.assignments_propositional.enqueue_decision_literal(y);
        solver.assignments_propositional.enqueue_decision_literal(z);

        solver.assignments_propositional.increase_decision_level();
        solver.assignments_propositional.enqueue_decision_literal(a);

        let lbd = calculate_lbd(&[a, x, y, z], &solver.assignments_propositional);

        assert_eq!(lbd, 2);
    }
}
