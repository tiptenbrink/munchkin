#![cfg(test)]
use crate::engine::test_helper::TestSolver;
use crate::propagators::cumulative::TimeTablePropagator;

// TODO: Add more tests here

/// A test case based on Example 4.3 from "Improving Scheduling by Learning - Andreas Schutt (2011)"
#[test]
fn time_table_updates_lower_bound() {
    let mut solver = TestSolver::default();
    let f = solver.new_variable(0, 14);
    let e = solver.new_variable(2, 4);
    let d = solver.new_variable(0, 2);
    let c = solver.new_variable(8, 9);
    let b = solver.new_variable(2, 3);
    let a = solver.new_variable(0, 1);

    let start_times = [a, b, c, d, e, f];
    let processing_times = [2, 6, 2, 2, 5, 6];
    let resource_usage = [1, 2, 4, 2, 2, 2];
    let capacity = 5;

    let _ = solver
        .new_propagator(TimeTablePropagator::new(
            start_times.into(),
            processing_times.into(),
            resource_usage.into(),
            capacity,
        ))
        .expect("Expected no conflict to occur");

    assert_eq!(solver.lower_bound(f), 10);
}

#[test]
fn time_table_updated_upper_bound() {
    let mut solver = TestSolver::default();
    let s1 = solver.new_variable(6, 6);
    let s2 = solver.new_variable(1, 8);

    let start_times = [s1, s2];
    let processing_times = [4, 3];
    let resource_usages = [1, 1];
    let capacity = 1;

    let _ = solver
        .new_propagator(TimeTablePropagator::new(
            start_times.into(),
            processing_times.into(),
            resource_usages.into(),
            capacity,
        ))
        .expect("Expected no conflict to occur");

    assert_eq!(solver.lower_bound(s2), 1);
    assert_eq!(solver.upper_bound(s2), 3);
    assert_eq!(solver.lower_bound(s1), 6);
    assert_eq!(solver.upper_bound(s1), 6);
}

#[test]
fn time_table_detects_conflict() {
    let mut solver = TestSolver::default();
    let s1 = solver.new_variable(1, 3);
    let s2 = solver.new_variable(3, 4);

    let start_times = [s1, s2];
    let processing_times = [4, 2];
    let resource_usages = [1, 1];
    let capacity = 1;

    let _ = solver
        .new_propagator(TimeTablePropagator::new(
            start_times.into(),
            processing_times.into(),
            resource_usages.into(),
            capacity,
        ))
        .expect_err("Expected conflict to be detected");
}
