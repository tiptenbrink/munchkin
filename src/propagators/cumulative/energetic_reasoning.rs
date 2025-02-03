#![allow(unused, reason = "this file is a skeleton for the assignment")]

use crate::basic_types::PropagationStatusCP;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

pub(crate) struct EnergeticReasoningPropagator<Var> {
    start_times: Box<[Var]>,
    durations: Box<[u32]>,
    resource_requirements: Box<[u32]>,
    resource_capacity: u32,
    // TODO: you can add more fields here!
}

impl<Var> EnergeticReasoningPropagator<Var> {
    pub(crate) fn new(
        start_times: Box<[Var]>,
        durations: Box<[u32]>,
        resource_requirements: Box<[u32]>,
        resource_capacity: u32,
    ) -> Self {
        EnergeticReasoningPropagator {
            start_times,
            durations,
            resource_requirements,
            resource_capacity,
        }
    }
}

impl<Var: IntegerVariable + 'static> Propagator for EnergeticReasoningPropagator<Var> {
    fn name(&self) -> &str {
        "EnergeticReasoning"
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

    use super::*;

    // TODO: Add tests here.

    /// A test case based on Figure 1 from "A quadratic edge-finding filtering algorithm for cumulative resource constraints - Kameugne et al. (2014)"
    fn energetic_reasoning_updates_lower_bound() {
        let mut solver = TestSolver::default();

        let a = solver.new_variable(1, 4);
        let b = solver.new_variable(5, 5);
        let c = solver.new_variable(1, 7);
        let d = solver.new_variable(1, 5);
        let e = solver.new_variable(5, 6);
        let f = solver.new_variable(0, 9);

        let start_times = [a, b, c, d, e, f];
        let processing_times = [4, 2, 1, 3, 1, 7];
        let resource_usages = [2, 2, 2, 1, 1, 1];
        let capacity = 3;

        let _ = solver
            .new_propagator(EnergeticReasoningPropagator::new(
                start_times.into(),
                processing_times.into(),
                resource_usages.into(),
                capacity,
            ))
            .expect("Expected no conflict to occur");
        assert_eq!(solver.lower_bound(f), 5);
    }

    /// A test case based on Figure 1 from "A quadratic edge-finding filtering algorithm for cumulative resource constraints - Kameugne et al. (2014)"
    fn energetic_reasoning_updates_upper_bound() {
        let mut solver = TestSolver::default();

        let a = solver.new_variable(9, 12);
        let b = solver.new_variable(9, 11);
        let c = solver.new_variable(9, 15);
        let d = solver.new_variable(9, 13);
        let e = solver.new_variable(10, 11);
        let f = solver.new_variable(0, 10);

        let start_times = [a, b, c, d, e, f];
        let processing_times = [4, 2, 1, 3, 1, 7];
        let resource_usages = [2, 2, 2, 1, 1, 1];
        let capacity = 3;

        let _ = solver
            .new_propagator(EnergeticReasoningPropagator::new(
                start_times.into(),
                processing_times.into(),
                resource_usages.into(),
                capacity,
            ))
            .expect("Expected no conflict to occur");
        assert_eq!(solver.upper_bound(f), 5);
    }

    /// A test case based on Figure 1 from "A quadratic edge-finding filtering algorithm for cumulative resource constraints - Kameugne et al. (2014)"
    fn energetic_reasoning_finds_conflict() {
        let mut solver = TestSolver::default();

        let a = solver.new_variable(1, 4);
        let b = solver.new_variable(5, 5);
        let c = solver.new_variable(1, 7);
        let d = solver.new_variable(1, 5);
        let e = solver.new_variable(5, 6);
        let f = solver.new_variable(0, 4);

        let start_times = [a, b, c, d, e, f];
        let processing_times = [4, 2, 1, 3, 1, 7];
        let resource_usages = [2, 2, 2, 1, 1, 1];
        let capacity = 3;

        let _ = solver
            .new_propagator(EnergeticReasoningPropagator::new(
                start_times.into(),
                processing_times.into(),
                resource_usages.into(),
                capacity,
            ))
            .expect_err("Expected conflict to be detected");
    }
}
