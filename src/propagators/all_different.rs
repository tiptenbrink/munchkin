#![allow(unused, reason = "this file is a skeleton for the assignment")]

use crate::basic_types::PropagationStatusCP;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

pub(crate) struct AllDifferentPropagator<Var> {
    variables: Box<[Var]>, // TODO: you can add more fields here!
}

impl<Var> AllDifferentPropagator<Var> {
    pub(crate) fn new(variables: Box<[Var]>) -> Self {
        Self { variables }
    }
}

impl<Var: IntegerVariable + 'static> Propagator for AllDifferentPropagator<Var> {
    fn name(&self) -> &str {
        "AllDifferent"
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
    use crate::engine::test_helper::TestSolver;

    use super::AllDifferentPropagator;

    #[test]
    fn test_simple_propagation() {
        let mut solver = TestSolver::default();

        let x1 = solver.new_variable(0, 3);
        let x2 = solver.new_variable(0, 3);
        let x3 = solver.new_variable(0, 3);
        let x4 = solver.new_variable(1, 2);
        let x5 = solver.new_variable(-2, 6);
        let x6 = solver.new_variable(1, 6);

        let variables = [x1, x2, x3, x4, x5, x6];

        let result = solver
            .new_propagator(AllDifferentPropagator::new(variables.into()))
            .expect("Expected no error");

        solver.assert_bounds(x6, 4, 6);
        for value in 0..=3 {
            assert!(solver.contains(x5, value));
        }
    }
}
