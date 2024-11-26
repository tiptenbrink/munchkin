use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::variables::IntegerVariable;

/// Propagator for the constraint `reif => \sum x_i <= c`.
#[derive(Debug)]
pub(crate) struct LinearLessOrEqualPropagator<Var> {
    terms: Box<[Var]>,
    rhs: i32,
    // TODO: you can add more fields here!
}

impl<Var> LinearLessOrEqualPropagator<Var> {
    pub(crate) fn new(terms: Box<[Var]>, rhs: i32) -> Self {
        Self { terms, rhs }
    }
}

impl<Var: IntegerVariable + 'static> Propagator for LinearLessOrEqualPropagator<Var> {
    fn name(&self) -> &str {
        "LinearLeq"
    }

    fn initialise_at_root(
        &mut self,
        _context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!()
    }

    fn propagate(&self, _context: PropagationContextMut) -> PropagationStatusCP {
        todo!()
    }
}
