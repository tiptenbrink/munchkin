#![cfg(test)]
use crate::engine::test_helper::TestSolver;
use crate::propagators::arithmetic::maximum::MaximumPropagator;

#[test]
fn upper_bound_of_rhs_matches_maximum_upper_bound_of_array_at_initialise() {
    let mut solver = TestSolver::default();

    let a = solver.new_variable(1, 3);
    let b = solver.new_variable(1, 4);
    let c = solver.new_variable(1, 5);

    let rhs = solver.new_variable(1, 10);

    let _ = solver
        .new_propagator(MaximumPropagator::new([a, b, c].into(), rhs))
        .expect("no empty domain");

    solver.assert_bounds(rhs, 1, 5);
}

#[test]
fn lower_bound_of_rhs_is_maximum_of_lower_bounds_in_array() {
    let mut solver = TestSolver::default();

    let a = solver.new_variable(3, 10);
    let b = solver.new_variable(4, 10);
    let c = solver.new_variable(5, 10);

    let rhs = solver.new_variable(1, 10);

    let _ = solver
        .new_propagator(MaximumPropagator::new([a, b, c].into(), rhs))
        .expect("no empty domain");

    solver.assert_bounds(rhs, 5, 10);
}

#[test]
fn upper_bound_of_all_array_elements_at_most_rhs_max_at_initialise() {
    let mut solver = TestSolver::default();

    let array = (1..=5)
        .map(|idx| solver.new_variable(1, 4 + idx))
        .collect::<Box<_>>();

    let rhs = solver.new_variable(1, 3);

    let _ = solver
        .new_propagator(MaximumPropagator::new(array.clone(), rhs))
        .expect("no empty domain");

    for var in array.iter() {
        solver.assert_bounds(*var, 1, 3);
    }
}

#[test]
fn single_variable_propagate() {
    let mut solver = TestSolver::default();

    let array = (1..=5)
        .map(|idx| solver.new_variable(1, 1 + 10 * idx))
        .collect::<Box<_>>();

    let rhs = solver.new_variable(45, 60);

    let _ = solver
        .new_propagator(MaximumPropagator::new(array.clone(), rhs))
        .expect("no empty domain");

    solver.assert_bounds(*array.last().unwrap(), 45, 51);
    solver.assert_bounds(rhs, 45, 51);
}
