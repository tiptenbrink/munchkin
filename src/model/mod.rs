use clap::ValueEnum;

use crate::constraints;
use crate::options::SolverOptions;
use crate::variables::AffineView;
use crate::variables::DomainId;
use crate::variables::TransformableVariable;
use crate::Solver;

/// Builds up the model, from which a solver can be constructed.
#[derive(Clone, Debug, Default)]
pub struct Model {
    /// Every element denotes the bounds of the variable.
    variables: Vec<(String, i32, i32)>,
    /// The constraints in the model.
    constraints: Vec<Constraint>,
}

impl Model {
    /// Create a new interval variable.
    pub fn new_interval_variable(
        &mut self,
        name: impl ToString,
        lower_bound: i32,
        upper_bound: i32,
    ) -> IntVariable {
        let id = self.variables.len();

        self.variables
            .push((name.to_string(), lower_bound, upper_bound));

        IntVariable {
            scale: 1,
            offset: 0,
            id,
        }
    }

    /// Add a constraint to the model.
    ///
    /// It is important to only use constraints with variables created on the same instance of
    /// [`Model`].
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Create a solver instance from this model.
    pub fn into_solver(
        self,
        solver_options: SolverOptions,
        use_global_propagator: impl Fn(Globals) -> bool,
    ) -> (Solver, VariableMap) {
        let mut solver = Solver::with_options(solver_options);

        let solver_variables: VariableMap = self
            .variables
            .into_iter()
            .map(|(name, lower_bound, upper_bound)| {
                (
                    AffineView::from(solver.new_named_bounded_integer(
                        lower_bound,
                        upper_bound,
                        name.clone(),
                    )),
                    name,
                )
            })
            .collect();

        let to_solver_variable =
            |int_var: IntVariable| solver_variables.to_solver_variable(int_var);

        for constraint in self.constraints {
            match constraint {
                Constraint::Circuit(variables) if use_global_propagator(Globals::Circuit) => {
                    let variables: Vec<_> = variables.into_iter().map(to_solver_variable).collect();

                    let _ = solver
                        .add_constraint(constraints::circuit(variables))
                        .post();
                }
                Constraint::Circuit(variables) => {
                    let variables: Vec<_> = variables.into_iter().map(to_solver_variable).collect();

                    let _ = solver
                        .add_constraint(constraints::circuit_decomposed(
                            variables,
                            !use_global_propagator(Globals::AllDifferent),
                            !use_global_propagator(Globals::Element),
                        ))
                        .post();
                }
                Constraint::Element { array, index, rhs } => {
                    let index = to_solver_variable(index);
                    let rhs = to_solver_variable(rhs);

                    let array: Vec<_> = array
                        .into_iter()
                        .map(|element| {
                            AffineView::from(solver.new_bounded_integer(element, element))
                        })
                        .collect();

                    if use_global_propagator(Globals::Element) {
                        let _ = solver
                            .add_constraint(constraints::element(index, array, rhs))
                            .post();
                    } else {
                        let _ = solver
                            .add_constraint(constraints::element_decomposition(index, array, rhs))
                            .post();
                    }
                }
                Constraint::LinearEqual { terms, rhs } => {
                    let terms: Vec<_> = terms.into_iter().map(to_solver_variable).collect();

                    let _ = solver
                        .add_constraint(constraints::equals(terms, rhs))
                        .post();
                }
            }
        }

        (solver, solver_variables)
    }
}

/// The constraints which can be used in [`Model`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Constraint {
    Circuit(Vec<IntVariable>),
    Element {
        array: Vec<i32>,
        index: IntVariable,
        rhs: IntVariable,
    },
    LinearEqual {
        terms: Vec<IntVariable>,
        rhs: i32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntVariable {
    /// The scale w.r.t. the underlying domain.
    scale: i32,
    /// The offset w.r.t. the underyling domain.
    offset: i32,
    /// The variable id.
    id: usize,
}

impl IntVariable {
    pub fn scaled(&self, scale: i32) -> IntVariable {
        IntVariable {
            scale: self.scale * scale,
            offset: self.offset,
            id: self.id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariableMap(Vec<AffineView<DomainId>>, Vec<String>);

impl FromIterator<(AffineView<DomainId>, String)> for VariableMap {
    fn from_iter<T: IntoIterator<Item = (AffineView<DomainId>, String)>>(iter: T) -> Self {
        let (variables, names) = iter.into_iter().unzip();
        VariableMap(variables, names)
    }
}

impl VariableMap {
    pub fn to_solver_variable(&self, int_var: IntVariable) -> AffineView<DomainId> {
        self.0[int_var.id]
            .scaled(int_var.scale)
            .offset(int_var.offset)
    }

    pub fn to_solver_variables<'this, I>(
        &'this self,
        int_vars: I,
    ) -> impl Iterator<Item = AffineView<DomainId>> + 'this
    where
        I: IntoIterator<Item = IntVariable> + 'this,
    {
        int_vars.into_iter().map(|var| self.to_solver_variable(var))
    }

    pub fn get_name(&self, int_var: IntVariable) -> String {
        let mut domain_name = self.1[int_var.id].clone();

        if int_var.scale != 1 {
            domain_name = format!("{} * {}", int_var.scale, domain_name);
        }

        if int_var.offset < 0 {
            domain_name = format!("{} - {}", domain_name, -int_var.offset);
        }

        if int_var.offset > 0 {
            domain_name = format!("{} + {}", domain_name, int_var.offset);
        }

        domain_name
    }
}

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Globals {
    Circuit,
    Element,
    AllDifferent,
}
