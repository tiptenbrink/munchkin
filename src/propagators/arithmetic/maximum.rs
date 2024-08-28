use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;
use crate::engine::propagation::PropagationContextMut;
use crate::engine::propagation::Propagator;
use crate::engine::propagation::PropagatorInitialisationContext;
use crate::variables::IntegerVariable;

/// Propagator which enforces `max(array) = rhs`.
#[derive(Debug)]
pub(crate) struct MaximumPropagator<ArrayVar, RhsVar> {
    array: Box<[ArrayVar]>,
    rhs: RhsVar,
    // TODO: you can add more fields here!
}

impl<ArrayVar, RhsVar> MaximumPropagator<ArrayVar, RhsVar> {
    pub(crate) fn new(array: Box<[ArrayVar]>, rhs: RhsVar) -> Self {
        Self { array, rhs }
    }
}

impl<ArrayVar, RhsVar> Propagator for MaximumPropagator<ArrayVar, RhsVar>
where
    ArrayVar: IntegerVariable + 'static,
    RhsVar: IntegerVariable + 'static,
{
    fn name(&self) -> &str {
        "Maximum"
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
