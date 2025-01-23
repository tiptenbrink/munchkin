#![allow(unused, reason = "this file is a skeleton for the assignment")]

use crate::basic_types::PropagationStatusCP;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

pub(crate) struct CumulativePropagator<Var> {
    start_times: Box<[Var]>,
    durations: Vec<u32>,
    resource_requirements: Vec<u32>,
    resource_capacity: u32,
    // TODO: you can add more fields here!
}

impl<Var> CumulativePropagator<Var> {
    pub(crate) fn new(
        start_times: Box<[Var]>,
        durations: &[u32],
        resource_requirements: &[u32],
        resource_capacity: u32,
    ) -> Self {
        CumulativePropagator {
            start_times,
            durations: durations.to_vec(),
            resource_requirements: resource_requirements.to_vec(),
            resource_capacity,
        }
    }
}

impl<Var: IntegerVariable + 'static> Propagator for CumulativePropagator<Var> {
    fn name(&self) -> &str {
        "Cumulative"
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
    use super::CumulativePropagator;
    use crate::engine::test_helper::TestSolver;

    #[test]
    fn test_simple_propagation() {
        let mut solver = TestSolver::default();

        let start_time_1 = solver.new_variable(5, 5);
        let start_time_2 = solver.new_variable(6, 9);

        let durations = [3, 2];
        let resource_requirements = [1, 1];
        let resource_capacity = 1;

        let _ = solver
            .new_propagator(CumulativePropagator::new(
                [start_time_1, start_time_2].into(),
                &durations,
                &resource_requirements,
                resource_capacity,
            ))
            .expect("Expected no conflict here");
        solver.assert_bounds(start_time_2, 8, 9);
    }

    #[test]
    fn test_simple_conflict() {
        let mut solver = TestSolver::default();

        let start_time_1 = solver.new_variable(5, 5);
        let start_time_2 = solver.new_variable(6, 7);

        let durations = [3, 2];
        let resource_requirements = [1, 1];
        let resource_capacity = 1;

        let _ = solver
            .new_propagator(CumulativePropagator::new(
                [start_time_1, start_time_2].into(),
                &durations,
                &resource_requirements,
                resource_capacity,
            ))
            .expect_err("Expected conflict at the root level");
    }
}
