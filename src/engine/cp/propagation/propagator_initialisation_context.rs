use super::PropagationContext;
use crate::engine::cp::domain_events::DomainEvents;
use crate::engine::cp::propagation::LocalId;
#[cfg(doc)]
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorId;
use crate::engine::cp::propagation::PropagatorVarId;
use crate::engine::cp::AssignmentsInteger;
use crate::engine::cp::WatchListCP;
use crate::engine::cp::WatchListPropositional;
use crate::engine::cp::Watchers;
use crate::engine::cp::WatchersPropositional;
use crate::engine::sat::AssignmentsPropositional;
use crate::engine::variables::IntegerVariable;
use crate::engine::variables::Literal;

/// [`PropagatorInitialisationContext`] is used when [`Propagator`]s are initialised after creation.
///
/// It represents a communication point between the [`Solver`] and the [`Propagator`].
/// Propagators use the [`PropagatorInitialisationContext`] to register to domain changes
/// of variables and to retrieve the current bounds of variables.
#[derive(Debug)]
pub struct PropagatorInitialisationContext<'a> {
    watch_list: &'a mut WatchListCP,
    watch_list_propositional: &'a mut WatchListPropositional,
    propagator_id: PropagatorId,
    next_local_id: LocalId,

    context: PropagationContext<'a>,
}

impl PropagatorInitialisationContext<'_> {
    pub(crate) fn new<'a>(
        watch_list: &'a mut WatchListCP,
        watch_list_propositional: &'a mut WatchListPropositional,
        propagator_id: PropagatorId,
        assignments_integer: &'a AssignmentsInteger,
        assignments_propositional: &'a AssignmentsPropositional,
    ) -> PropagatorInitialisationContext<'a> {
        PropagatorInitialisationContext {
            watch_list,
            watch_list_propositional,
            propagator_id,
            next_local_id: LocalId::from(0),

            context: PropagationContext::new(assignments_integer, assignments_propositional),
        }
    }

    /// Subscribes the propagator to the given [`DomainEvents`].
    ///
    /// The domain events determine when [`Propagator::notify()`] will be called on the propagator.
    /// The [`LocalId`] is internal information related to the propagator,
    /// which is used when calling [`Propagator::notify()`] to identify the variable.
    ///
    /// Each variable *must* have a unique [`LocalId`]. Most often this would be its index of the
    /// variable in the internal array of variables.
    ///
    /// Note that the [`LocalId`] is used to differentiate between [`DomainId`]s and
    /// [`AffineView`]s.
    pub fn register<Var: IntegerVariable>(
        &mut self,
        var: Var,
        domain_events: DomainEvents,
        local_id: LocalId,
    ) -> Var {
        let propagator_var = PropagatorVarId {
            propagator: self.propagator_id,
            variable: local_id,
        };

        self.next_local_id = self.next_local_id.max(LocalId::from(local_id.unpack() + 1));

        let mut watchers = Watchers::new(propagator_var, self.watch_list);
        var.watch_all(&mut watchers, domain_events.get_int_events());

        var
    }

    pub fn register_literal(
        &mut self,
        var: Literal,
        domain_events: DomainEvents,
        local_id: LocalId,
    ) -> Literal {
        let propagator_var = PropagatorVarId {
            propagator: self.propagator_id,
            variable: local_id,
        };

        self.next_local_id = self.next_local_id.max(LocalId::from(local_id.unpack() + 1));

        let mut watchers =
            WatchersPropositional::new(propagator_var, self.watch_list_propositional);
        watchers.watch_all(var, domain_events.get_bool_events());

        var
    }

    pub fn get_next_local_id(&self) -> LocalId {
        self.next_local_id
    }
}

mod private {
    use super::*;
    use crate::engine::cp::propagation::propagation_context::HasAssignments;

    impl HasAssignments for PropagatorInitialisationContext<'_> {
        fn assignments_integer(&self) -> &AssignmentsInteger {
            self.context.assignments_integer()
        }

        fn assignments_propositional(&self) -> &AssignmentsPropositional {
            self.context.assignments_propositional()
        }
    }
}
