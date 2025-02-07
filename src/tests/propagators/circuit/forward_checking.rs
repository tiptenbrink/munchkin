#![cfg(test)]
use crate::engine::test_helper::TestSolver;
use crate::propagators::circuit::ForwardCheckingCircuitPropagator;

#[test]
fn detects_failure() {
    let mut solver = TestSolver::default();

    let a = solver.new_variable(1, 1);
    let b = solver.new_variable(0, 0);
    let c = solver.new_variable(0, 1);

    let _ = solver
        .new_propagator(ForwardCheckingCircuitPropagator::new([a, b, c].into()))
        .expect_err("Expected circuit to detect cycle");
}

#[test]
fn detects_simple_prevent() {
    let mut solver = TestSolver::default();

    let a = solver.new_variable(1, 1);
    let b = solver.new_variable(0, 2);
    let c = solver.new_variable(0, 2);

    let _ = solver
        .new_propagator(ForwardCheckingCircuitPropagator::new([a, b, c].into()))
        .expect("Expected circuit to not detect a conflict");

    solver.assert_bounds(b, 2, 2);
    // No self-loops
    assert!(!solver.contains(c, 2));
}
