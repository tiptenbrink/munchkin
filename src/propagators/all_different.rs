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

    #[test]
    fn test_simple_propagation() {
        let mut solver = TestSolver::default();

        let x = solver.new_variable(5, 5);
        let y = solver.new_variable(6, 6);
        let y = solver.new_variable(5, 7);

        // let _ = solver.new_propagator();
    }
}
