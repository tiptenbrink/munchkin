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

impl<Var: IntegerVariable> LinearNotEqualPropagator<Var> {
    fn get_fixed_term_count(&self, context: PropagationContext<'_>) -> usize {
        self.terms
            .iter()
            .filter(|term| context.is_fixed(*term))
            .count()
    }

    fn get_conflict(&self, context: PropagationContext<'_>) -> PropositionalConjunction {
        self.terms
            .iter()
            .map(|term| {
                let value = context.lower_bound(term);
                predicate![term == value]
            })
            .collect()
    }

    fn get_fixed_terms<'this, 'context>(
        &'this self,
        context: PropagationContext<'context>,
    ) -> impl Iterator<Item = &'this Var> + 'context
    where
        'this: 'context,
    {
        self.terms
            .iter()
            .filter(move |term| context.is_fixed(*term))
    }
}

impl<Var: IntegerVariable + 'static> Propagator for LinearNotEqualPropagator<Var> {
    fn name(&self) -> &str {
        "LinearNe"
    }

    fn propagate(&self, mut context: PropagationContextMut) -> PropagationStatusCP {
        let fixed_count = self.get_fixed_term_count(context.as_readonly());

        let fixed_lhs: i32 = self
            .get_fixed_terms(context.as_readonly())
            .map(|term| context.lower_bound(term))
            .sum();

        if fixed_count == self.terms.len() && fixed_lhs == self.rhs {
            return Err(self.get_conflict(context.as_readonly()).into());
        }

        if fixed_count + 1 == self.terms.len() {
            let unfixed_term = self
                .terms
                .iter()
                .find(|term| !context.is_fixed(*term))
                .expect("there should be exactly 1 unfixed term");

            let reason: PropositionalConjunction = self
                .get_fixed_terms(context.as_readonly())
                .map(|term| {
                    let value = context.lower_bound(term);
                    predicate![term == value]
                })
                .collect();

            context.remove(unfixed_term, self.rhs - fixed_lhs, reason)?;
        }

        Ok(())
    }

    fn detect_inconsistency(
        &self,
        context: PropagationContext,
    ) -> Option<PropositionalConjunction> {
        let fixed_count = self.get_fixed_term_count(context);

        if fixed_count < self.terms.len() {
            // There are still unfixed terms. This means the constraint is not violated.
            return None;
        }

        // All terms are assigned at this point. So the lower bound equals the upper bound of every
        // term.
        let lhs: i32 = self
            .terms
            .iter()
            .map(|term| context.lower_bound(term))
            .sum();

        if lhs == self.rhs {
            Some(self.get_conflict(context))
        } else {
            None
        }
    }

    fn initialise_at_root(
        &mut self,
        context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        for term in self.terms.iter() {
            context.register(term.clone(), DomainEvents::ASSIGN);
        }

        Ok(())
    }
}
