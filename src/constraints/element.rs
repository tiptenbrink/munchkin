use super::Constraint;
use crate::variables::IntegerVariable;

/// Creates the [element](https://sofdem.github.io/gccat/gccat/Celement.html) [`Constraint`] which states that `array[index] = rhs`.
pub fn element<ElementVar: IntegerVariable + 'static>(
    index: impl IntegerVariable + 'static,
    array: impl Into<Box<[ElementVar]>>,
    rhs: impl IntegerVariable + 'static,
) -> impl Constraint {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::{constraints::Constraint, engine::test_helper::TestSolver};

    use super::element;

    #[test]
    fn test_simple_propagation_index() {
        let mut solver = TestSolver::default();

        let x = solver.new_variable(0, 10);
        let y = solver.new_variable(0, 0);

        let index = solver.new_variable(0, 1);

        let rhs = solver.new_variable(5, 5);

        let mut propagator = element(index, [x, y], rhs)
            .post_test(&mut solver)
            .expect("No root level error");

        // We know that the index can not point to y (since it is fixed at 0)
        // and the rhs is fixed at 5 so the index should be propagated to 0
        solver.assert_bounds(index, 0, 0);

        let result = solver.propagate(&mut propagator);
        assert!(result.is_ok());

        // And the value of x should be fixed to 5 now
        solver.assert_bounds(x, 5, 5);
    }
}
