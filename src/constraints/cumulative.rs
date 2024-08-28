use std::fmt::Debug;

use super::Constraint;
use crate::propagators::cumulative::CumulativePropagator;
use crate::pumpkin_assert_simple;
use crate::variables::IntegerVariable;

/// Creates the [Cumulative](https://sofdem.github.io/gccat/gccat/Ccumulative.html) [`Constraint`].
/// This constraint ensures that at no point in time, the cumulative resource usage of the tasks
/// exceeds `bound`.
pub fn cumulative<Var: IntegerVariable + 'static + Debug>(
    start_times: &[Var],
    durations: &[u32],
    resource_requirements: &[u32],
    resource_capacity: u32,
) -> impl Constraint {
    pumpkin_assert_simple!(
        start_times.len() == durations.len() && durations.len() == resource_requirements.len(),
        "The number of start variables, durations and resource requirements should be the same!"
    );

    CumulativePropagator::new(
        start_times.into(),
        durations,
        resource_requirements,
        resource_capacity,
    )
}
