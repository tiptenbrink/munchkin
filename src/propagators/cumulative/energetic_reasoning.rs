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
