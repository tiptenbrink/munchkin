use crate::{
    basic_types::PropagationStatusCP,
    engine::propagation::{PropagationContextMut, Propagator, PropagatorInitialisationContext},
    predicates::PropositionalConjunction,
    variables::IntegerVariable,
};

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
