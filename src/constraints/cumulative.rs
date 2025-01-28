use std::fmt::Debug;

use super::Constraint;
use crate::constraints;
use crate::munchkin_assert_simple;
use crate::propagators::cumulative::CumulativePropagator;
use crate::variables::IntegerVariable;
use crate::variables::Literal;
use crate::ConstraintOperationError;
use crate::Solver;

/// Creates the [Cumulative](https://sofdem.github.io/gccat/gccat/Ccumulative.html) [`Constraint`].
/// This constraint ensures that at no point in time, the cumulative resource usage of the tasks
/// exceeds `bound`.
pub fn cumulative<Var: IntegerVariable + 'static + Debug>(
    start_times: impl Into<Box<[Var]>>,
    durations: &[u32],
    resource_requirements: &[u32],
    resource_capacity: u32,
) -> impl Constraint {
    let start_times = start_times.into();

    munchkin_assert_simple!(
        start_times.len() == durations.len() && durations.len() == resource_requirements.len(),
        "The number of start variables, durations and resource requirements should be the same!"
    );

    CumulativePropagator::new(
        start_times,
        durations,
        resource_requirements,
        resource_capacity,
    )
}

/// Implements the cumulative constraint through a decomposition.
pub fn cumulative_decomposition<Var: IntegerVariable + 'static + Debug>(
    start_times: impl Into<Box<[Var]>>,
    durations: impl Into<Box<[u32]>>,
    resource_requirements: impl Into<Box<[u32]>>,
    resource_capacity: u32,
) -> impl Constraint {
    CumulativeDecomposition {
        start_times: start_times.into(),
        durations: durations.into(),
        resource_requirements: resource_requirements.into(),
        resource_capacity,
    }
}

struct CumulativeDecomposition<Var> {
    start_times: Box<[Var]>,
    durations: Box<[u32]>,
    resource_requirements: Box<[u32]>,
    resource_capacity: u32,
}

impl<Var: IntegerVariable + 'static> Constraint for CumulativeDecomposition<Var> {
    fn post(self, solver: &mut Solver) -> Result<(), ConstraintOperationError> {
        let horizon: u32 = self.durations.iter().sum();

        for timepoint in 0..=horizon {
            let mut usages = vec![];

            for task in 0..self.start_times.len() {
                let resource_requirement = self.resource_requirements[task] as i32;
                if resource_requirement == 0 {
                    continue;
                }

                let is_active_at_timepoint = solver.new_literal();
                let usage_of_task_at_current_timepoint =
                    solver.new_sparse_integer([0, resource_requirement]);

                // If the timepoint starts after or ends before `timepoint`, the resource usage
                // will be 0.
                solver
                    .add_constraint(constraints::equals([usage_of_task_at_current_timepoint], 0))
                    .reify(!is_active_at_timepoint)?;

                let duration = self.durations[task];

                let ends_before = if duration > timepoint {
                    solver.get_false_literal()
                } else {
                    let literal = solver.new_literal();

                    // ends_before <-> start[task] + duration[task] <= timepoint
                    solver
                        .add_constraint(constraints::less_than_or_equals(
                            [self.start_times[task].clone()],
                            (timepoint - duration) as i32,
                        ))
                        .reify(literal)?;

                    literal
                };

                let starts_after = if timepoint + duration > horizon {
                    solver.get_false_literal()
                } else {
                    let literal = solver.new_literal();

                    // starts_after <-> start[task] > timepoint
                    // starts_after <-> start[task] >= timepoint + 1
                    // starts_after <-> -start[task] <= -timepoint - 1
                    solver
                        .add_constraint(constraints::less_than_or_equals(
                            [self.start_times[task].scaled(-1)],
                            -(timepoint as i32) - 1,
                        ))
                        .reify(literal)?;

                    literal
                };

                // !is_active_at_timepoint <-> (ends_before \/ starts_after)
                solver
                    .add_constraint(constraints::clause([ends_before, starts_after]))
                    .reify(!is_active_at_timepoint)?;

                usages.push(usage_of_task_at_current_timepoint);
            }

            solver
                .add_constraint(constraints::less_than_or_equals(
                    usages,
                    self.resource_capacity as i32,
                ))
                .post()?;
        }

        Ok(())
    }

    fn implied_by(self, _: &mut Solver, _: Literal) -> Result<(), ConstraintOperationError> {
        todo!("implement cumulative decomposition with half-reification")
    }
}
