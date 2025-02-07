#![allow(unused, reason = "this file is a skeleton for the assignment")]

use crate::basic_types::PropagationStatusCP;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

pub(crate) struct DfsCircuitPropagator<Var> {
    successor: Box<[Var]>,
    // TODO: you can add more fields here!
}

impl<Var> DfsCircuitPropagator<Var> {
    pub(crate) fn new(successor: Box<[Var]>) -> Self {
        Self { successor }
    }
}

impl<Var: IntegerVariable + 'static> Propagator for DfsCircuitPropagator<Var> {
    fn name(&self) -> &str {
        "DfsCircuit"
    }

    fn propagate(&self, _context: PropagationContextMut) -> PropagationStatusCP {
        todo!()
    }

    fn initialise_at_root(
        &mut self,
        _: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::test_helper::TestSolver;

    use super::DfsCircuitPropagator;

    #[test]
    fn detects_failure() {
        let mut solver = TestSolver::default();

        let a = solver.new_variable(1, 1);
        let b = solver.new_variable(0, 0);
        let c = solver.new_variable(0, 1);

        let _ = solver
            .new_propagator(DfsCircuitPropagator::new([a, b, c].into()))
            .expect_err("Expected circuit to detect cycle");
    }

    #[test]
    fn detects_simple_prevent() {
        let mut solver = TestSolver::default();

        let a = solver.new_variable(1, 1);
        let b = solver.new_variable(0, 2);
        let c = solver.new_variable(0, 2);

        let _ = solver
            .new_propagator(DfsCircuitPropagator::new([a, b, c].into()))
            .expect("Expected circuit to not detect a conflict");

        solver.assert_bounds(b, 2, 2);
        // No self-loops
        assert!(!solver.contains(c, 2));
    }

    #[test]
    // An example based on Figure 4 in "Explaining circuit propagation - Francis & Stuckey (2013)"
    fn detect_simple_dfs() {
        let mut solver = TestSolver::default();

        let a = solver.new_sparse_variable(&[1, 4, 5]);
        let b = solver.new_sparse_variable(&[2, 3]);
        let c = solver.new_sparse_variable(&[0]);
        let d = solver.new_sparse_variable(&[2]);
        let e = solver.new_sparse_variable(&[1, 3]);
        let f = solver.new_sparse_variable(&[0, 6]);
        let g = solver.new_sparse_variable(&[3, 4]);

        let _ = solver
            .new_propagator(DfsCircuitPropagator::new([a, b, c, d, e, f, g].into()))
            .expect("{Expected no error}");

        assert!(!solver.contains(f, 0));
        assert!(!solver.contains(g, 3));
    }
}
