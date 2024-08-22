use crate::{
    basic_types::PropagationStatusCP,
    engine::propagation::{PropagationContextMut, Propagator},
    predicates::PropositionalConjunction,
};

pub struct AllDifferentPropagator {
    // TODO
}

impl Propagator for AllDifferentPropagator {
    fn name(&self) -> &str {
        "AllDifferent"
    }

    fn propagate(&self, context: PropagationContextMut) -> PropagationStatusCP {
        todo!()
    }

    fn initialise_at_root(
        &mut self,
        _: &mut crate::engine::propagation::PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!()
    }
}
