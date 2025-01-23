use super::Constraint;
use crate::constraints;
use crate::predicate;
use crate::propagators::circuit::CircuitPropagator;
use crate::variables::AffineView;
use crate::variables::DomainId;
use crate::variables::IntegerVariable;
use crate::variables::Literal;
use crate::variables::TransformableVariable;
use crate::ConstraintOperationError;
use crate::Solver;

/// Creates the [`Constraint`] that enforces that the assigned successors form a circuit
/// (i.e. a path which visits each vertex once and starts and ends at the same node).
///
/// `successor[i] = j` means that `j` is the successor of `i`.
pub fn circuit<Var: IntegerVariable + 'static>(
    successor: impl Into<Box<[Var]>>,
) -> impl Constraint {
    CircuitPropagator::new(successor.into())
}

/// Creates a [`Constraint`] equivalent to [`circuit`], but using a decomposition rather than a
/// global propagator.
///
/// Note that the decomposition is exponential in the number of variables.
pub fn circuit_decomposed(
    successor: impl Into<Box<[AffineView<DomainId>]>>,
    use_all_different_decomposition: bool,
    use_element_encoding: bool,
) -> impl Constraint {
    DecomposedCircuit {
        successors: successor.into(),
        use_all_different_decomposition,
        use_element_encoding,
    }
}

struct DecomposedCircuit {
    successors: Box<[AffineView<DomainId>]>,
    use_all_different_decomposition: bool,
    use_element_encoding: bool,
}

impl Constraint for DecomposedCircuit {
    fn post(self, solver: &mut Solver) -> Result<(), ConstraintOperationError> {
        let DecomposedCircuit {
            successors,
            use_all_different_decomposition,
            use_element_encoding,
        } = self;

        let min = successors
            .iter()
            .map(|var| solver.lower_bound(var))
            .min()
            .unwrap();
        assert_eq!(0, min);

        let max = successors
            .iter()
            .map(|var| solver.upper_bound(var))
            .max()
            .unwrap();
        assert_eq!(i32::try_from(successors.len() - 1).unwrap(), max);

        let order: Box<[_]> = (0..=max)
            .map(|i| {
                let ub = if i == 0 { 0 } else { max };

                AffineView::from(solver.new_bounded_integer(0, ub))
            })
            .collect();

        if use_all_different_decomposition {
            solver
                .add_constraint(constraints::all_different_decomposition(successors.clone()))
                .post()?;
            solver
                .add_constraint(constraints::all_different_decomposition(order.clone()))
                .post()?;
        } else {
            solver
                .add_constraint(constraints::all_different(successors.clone()))
                .post()?;
            solver
                .add_constraint(constraints::all_different(order.clone()))
                .post()?;
        }

        for (idx, var) in successors.iter().enumerate() {
            let idx: i32 = idx.try_into().unwrap();

            solver
                .add_constraint(constraints::not_equals([var.clone()], idx))
                .post()?;
        }

        for (i, successor) in successors.iter().enumerate() {
            let succ_order = solver.new_bounded_integer(0, max);

            if use_element_encoding {
                solver
                    .add_constraint(constraints::element_decomposition(
                        successor.clone(),
                        order.clone(),
                        succ_order.into(),
                    ))
                    .post()?;
            } else {
                solver
                    .add_constraint(constraints::element(
                        successor.clone(),
                        order.clone(),
                        succ_order,
                    ))
                    .post()?;
            }

            let order_i_eq_max = solver.get_literal(predicate![order[i] == max]);

            solver
                .add_constraint(constraints::equals([succ_order], 0))
                .implied_by(order_i_eq_max)?;
            solver
                .add_constraint(constraints::equals(
                    [succ_order.into(), order[i].clone().scaled(-1)],
                    1,
                ))
                .implied_by(!order_i_eq_max)?;
        }

        Ok(())
    }

    fn implied_by(self, _: &mut Solver, _: Literal) -> Result<(), ConstraintOperationError> {
        todo!("implement half reification of decomposed circuit")
    }
}
