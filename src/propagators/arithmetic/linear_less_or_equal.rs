use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;
use crate::engine::cp::domain_events::DomainEvents;
use crate::engine::cp::propagation::PropagationContext;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::engine::cp::propagation::ReadDomains;
use crate::predicate;
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

impl<Var: IntegerVariable> LinearLessOrEqualPropagator<Var> {
    fn get_optimistic_lhs(&self, context: PropagationContext<'_>) -> i32 {
        self.terms
            .iter()
            .map(|term| context.lower_bound(term))
            .sum()
    }
}

impl<Var: IntegerVariable + 'static> Propagator for LinearLessOrEqualPropagator<Var> {
    fn name(&self) -> &str {
        "LinearLeq"
    }

    fn initialise_at_root(
        &mut self,
        context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        for term in self.terms.iter() {
            context.register(term.clone(), DomainEvents::LOWER_BOUND);
        }

        Ok(())
    }

    fn detect_inconsistency(
        &self,
        context: PropagationContext,
    ) -> Option<PropositionalConjunction> {
        let optimistic_lhs = self.get_optimistic_lhs(context);

        if optimistic_lhs > self.rhs {
            let conflict = self
                .terms
                .iter()
                .map(|term| {
                    let value = context.lower_bound(term);
                    predicate![term >= value]
                })
                .collect();

            Some(conflict)
        } else {
            None
        }
    }

    fn propagate(&self, mut context: PropagationContextMut) -> PropagationStatusCP {
        let optimistic_lhs = self.get_optimistic_lhs(context.as_readonly());

        for (i, term) in self.terms.iter().enumerate() {
            let bound = self.rhs - (optimistic_lhs - context.lower_bound(term));

            if context.upper_bound(term) > bound {
                let reason: PropositionalConjunction = self
                    .terms
                    .iter()
                    .enumerate()
                    .filter_map(|(j, x_j)| {
                        if j != i {
                            Some(predicate![x_j >= context.lower_bound(x_j)])
                        } else {
                            None
                        }
                    })
                    .collect();

                context.set_upper_bound(term, bound, reason)?;
            }
        }

        Ok(())
    }
}
