use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;
use crate::engine::propagation::PropagationContextMut;
use crate::engine::propagation::Propagator;
use crate::engine::propagation::PropagatorInitialisationContext;

/// Propagator for the constraint `reif => \sum x_i <= c`.
#[derive(Debug)]
pub(crate) struct LinearLessOrEqualPropagator {
    // TODO
}

impl Propagator for LinearLessOrEqualPropagator {
    fn name(&self) -> &str {
        "LinearLeq"
    }

    fn initialise_at_root(
        &mut self,
        context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!()
    }

    fn propagate(&self, context: PropagationContextMut) -> PropagationStatusCP {
        todo!()
    }
}
