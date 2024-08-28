use crate::basic_types::PropagationStatusCP;
use crate::engine::propagation::PropagationContextMut;
use crate::engine::propagation::Propagator;
use crate::engine::propagation::PropagatorInitialisationContext;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

/// Propagator for constraint `element([x_1, \ldots, x_n], i, e)`, where `x_j` are
///  variables, `i` is an integer variable, and `e` is a variable, which holds iff `x_i = e`
///
/// Note that this propagator is 0-indexed
pub(crate) struct ElementPropagator<IndexVar, ArrayVar, RhsVar> {
    index: IndexVar,
    array: Box<[ArrayVar]>,
    rhs: RhsVar,
    // TODO: you can add more fields here!
}

impl<IndexVar, ArrayVar, RhsVar> ElementPropagator<IndexVar, ArrayVar, RhsVar> {
    pub(crate) fn new(index: IndexVar, array: Box<[ArrayVar]>, rhs: RhsVar) -> Self {
        ElementPropagator { index, array, rhs }
    }
}

impl<IndexVar, ArrayVar, RhsVar> Propagator for ElementPropagator<IndexVar, ArrayVar, RhsVar>
where
    IndexVar: IntegerVariable + 'static,
    ArrayVar: IntegerVariable + 'static,
    RhsVar: IntegerVariable + 'static,
{
    fn name(&self) -> &str {
        "Element"
    }

    fn propagate(&self, _context: PropagationContextMut) -> PropagationStatusCP {
        todo!()
    }

    fn initialise_at_root(
        &mut self,
        _context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{engine::test_helper::TestSolver, propagators::element::ElementPropagator};

    #[test]
    fn test_simple_propagation() {
        let mut solver = TestSolver::default();

        let x = solver.new_variable(0, 10);
        let y = solver.new_variable(0, 0);

        let index = solver.new_variable(0, 1);

        let rhs = solver.new_variable(5, 5);

        let mut propagator = solver
            .new_propagator(ElementPropagator::new(index, [x, y].into(), rhs))
            .expect("Expected no conflict here");

        // We know that the index can not point to y (since it is fixed at 0)
        // and the rhs is fixed at 5 so the index should be propagated to 0
        solver.assert_bounds(index, 0, 0);

        let result = solver.propagate(&mut propagator);
        assert!(result.is_ok());

        // And the value of x should be fixed to 5 now
        solver.assert_bounds(x, 5, 5);
    }

    #[test]
    fn test_simple_conflict() {
        let mut solver = TestSolver::default();

        let x = solver.new_variable(6, 10);
        let y = solver.new_variable(0, 4);

        let index = solver.new_variable(0, 1);

        let rhs = solver.new_variable(5, 5);

        // The instance we have provided is infeasible, the propagator should report a conflict
        let _ = solver
            .new_propagator(ElementPropagator::new(index, [x, y].into(), rhs))
            .expect_err("Expected conflict at the root level");
    }
}
