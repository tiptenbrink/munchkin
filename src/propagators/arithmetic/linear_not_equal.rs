use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;

use crate::engine::propagation::PropagationContextMut;
use crate::engine::propagation::Propagator;
use crate::engine::propagation::PropagatorInitialisationContext;

/// Propagator for the constraint `\sum x_i != rhs`, where `x_i` are
/// integer variables and `rhs` is an integer constant.
#[derive(Debug)]
pub(crate) struct LinearNotEqualPropagator {
    // TODO
}

impl Propagator for LinearNotEqualPropagator {
    fn name(&self) -> &str {
        "LinearNe"
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
