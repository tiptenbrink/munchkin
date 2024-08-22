use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;
use crate::engine::propagation::PropagationContextMut;
use crate::engine::propagation::Propagator;
use crate::engine::propagation::PropagatorInitialisationContext;

/// Propagator which enforces `max(array) = rhs`.
#[derive(Debug)]
pub(crate) struct MaximumPropagator {
    // TODO
}

impl Propagator for MaximumPropagator {
    fn name(&self) -> &str {
        "Maximum"
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
