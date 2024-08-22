use crate::{
    basic_types::PropagationStatusCP,
    engine::propagation::{PropagationContextMut, Propagator, PropagatorInitialisationContext},
    predicates::PropositionalConjunction,
};

pub(crate) struct CumulativePropagator {
    // TODO
}

impl Propagator for CumulativePropagator {
    fn name(&self) -> &str {
        "Cumulative"
    }

    fn propagate(&self, context: PropagationContextMut) -> PropagationStatusCP {
        todo!()
    }

    fn initialise_at_root(
        &mut self,
        context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!()
    }
}
