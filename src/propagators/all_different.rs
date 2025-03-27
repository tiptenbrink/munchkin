#![allow(unused, reason = "this file is a skeleton for the assignment")]

use crate::basic_types::PropagationStatusCP;
use crate::conjunction;
use crate::engine::cp::domain_events::DomainEvents;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::engine::cp::propagation::ReadDomains;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

pub(crate) struct AllDifferentPropagator<Var> {
    variables: Box<[Var]>, // TODO: you can add more fields here!
}

impl<Var> AllDifferentPropagator<Var> {
    pub(crate) fn new(variables: Box<[Var]>) -> Self {
        Self { variables }
    }
}

impl<Var: IntegerVariable + 'static> Propagator for AllDifferentPropagator<Var> {
    fn name(&self) -> &str {
        "AllDifferent"
    }

    fn propagate(&self, mut context: PropagationContextMut) -> PropagationStatusCP {
        for (idx1, variable) in self.variables.iter().enumerate() {
            let value = if context.is_fixed(variable) {
                context.lower_bound(variable)
            } else {
                continue;
            };

            for (idx2, other) in self.variables.iter().enumerate() {
                if idx1 == idx2 {
                    continue;
                }

                context.remove(other, value, conjunction!([variable == value]))?;
            }
        }

        Ok(())
    }

    fn initialise_at_root(
        &mut self,
        context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        for variable in self.variables.iter() {
            context.register(variable.clone(), DomainEvents::ASSIGN);
        }

        Ok(())
    }
}
