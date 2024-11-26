use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::variables::IntegerVariable;

/// Propagator for the constraint `\sum x_i != rhs`, where `x_i` are
/// integer variables and `rhs` is an integer constant.
#[derive(Debug)]
pub(crate) struct LinearNotEqualPropagator<Var> {
    terms: Box<[Var]>,
    rhs: i32,
    // TODO: you can add more fields here!
}

impl<Var> LinearNotEqualPropagator<Var> {
    pub(crate) fn new(terms: Box<[Var]>, rhs: i32) -> Self {
        Self { terms, rhs }
    }
}

impl<Var: IntegerVariable + 'static> Propagator for LinearNotEqualPropagator<Var> {
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
