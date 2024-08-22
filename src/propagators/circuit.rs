use crate::{
    basic_types::PropagationStatusCP,
    engine::propagation::{PropagationContextMut, Propagator, PropagatorInitialisationContext},
    predicates::PropositionalConjunction,
};

pub struct CircuitPropagator {
    // TODO
}

impl Propagator for CircuitPropagator {
    fn name(&self) -> &str {
        "Circuit"
    }

    fn propagate(&self, context: PropagationContextMut) -> PropagationStatusCP {
        todo!()
    }

    fn initialise_at_root(
        &mut self,
        _: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!()
    }
}
