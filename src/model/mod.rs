use std::fmt::Display;
use std::ops::Range;

use clap::ValueEnum;

use crate::constraints;
use crate::options::SolverOptions;
use crate::variables::AffineView;
use crate::variables::DomainId;
use crate::variables::TransformableVariable;
use crate::ConstraintOperationError;
use crate::Solver;

/// Builds up the model, from which a solver can be constructed.
#[derive(Clone, Debug, Default)]
pub struct Model {
    /// Every element denotes the bounds of the variable.
    variables: Vec<(String, i32, i32)>,
    /// Arrays of variables.
    arrays: Vec<(String, Range<usize>)>,
    /// The constraints in the model.
    constraints: Vec<Constraint>,
}

impl Model {
    /// Create a new interval variable.
    pub fn new_interval_variable(
        &mut self,
        name: impl Display,
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

    /// Create a new array of interval variables.
    pub fn new_interval_variable_array(
        &mut self,
        name: impl Display,
        lower_bound: i32,
        upper_bound: i32,
        len: usize,
    ) -> IntVariableArray {
        let id = self.arrays.len();

        let start = self.variables.len();
        (0..len).for_each(|i| {
            let _ = self.new_interval_variable(format!("{name}[{i}]"), lower_bound, upper_bound);
        });

        let end = self.variables.len();

        self.arrays.push((name.to_string(), start..end));

        IntVariableArray(id)
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

        let (variables, names): (Vec<_>, Vec<_>) = self
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
            .unzip();

        let solver_variables = VariableMap {
            variables,
            names,
            arrays: self.arrays,
        };

        let _ = add_constraints(
            self.constraints,
            &solver_variables,
            use_global_propagator,
            &mut solver,
        );

        (solver, solver_variables)
    }
}

fn add_constraints(
    constraints: Vec<Constraint>,
    solver_variables: &VariableMap,
    use_global_propagator: impl Fn(Globals) -> bool,
    solver: &mut Solver,
) -> Result<(), ConstraintOperationError> {
    let to_solver_variable = |int_var: IntVariable| solver_variables.to_solver_variable(int_var);

    for constraint in constraints {
        match constraint {
            Constraint::Circuit(variables) if use_global_propagator(Globals::Circuit) => {
                let variables: Vec<_> = variables.into_iter().map(to_solver_variable).collect();

                solver
                    .add_constraint(constraints::circuit(variables))
                    .post()?;
            }
            Constraint::Circuit(variables) => {
                let variables: Vec<_> = variables.into_iter().map(to_solver_variable).collect();

                solver
                    .add_constraint(constraints::circuit_decomposed(
                        variables,
                        !use_global_propagator(Globals::AllDifferent),
                        !use_global_propagator(Globals::Element),
                    ))
                    .post()?;
            }
            Constraint::Element { array, index, rhs } => {
                let index = to_solver_variable(index);
                let rhs = to_solver_variable(rhs);

                let array: Vec<_> = array
                    .into_iter()
                    .map(|element| AffineView::from(solver.new_bounded_integer(element, element)))
                    .collect();

                if use_global_propagator(Globals::Element) {
                    solver
                        .add_constraint(constraints::element(index, array, rhs))
                        .post()?;
                } else {
                    solver
                        .add_constraint(constraints::element_decomposition(index, array, rhs))
                        .post()?;
                }
            }
            Constraint::LinearEqual { terms, rhs } => {
                let terms: Vec<_> = terms.into_iter().map(to_solver_variable).collect();

                solver
                    .add_constraint(constraints::equals(terms, rhs))
                    .post()?;
            }
            Constraint::LinearLessEqual { terms, rhs } => {
                let terms: Vec<_> = terms.into_iter().map(to_solver_variable).collect();

                solver
                    .add_constraint(constraints::less_than_or_equals(terms, rhs))
                    .post()?;
            }
            Constraint::Cumulative {
                start_times,
                durations,
                resource_requirements,
                resource_capacity,
            } => {
                let start_times: Vec<_> = start_times.into_iter().map(to_solver_variable).collect();

                if use_global_propagator(Globals::Cumulative) {
                    solver
                        .add_constraint(constraints::cumulative(
                            start_times,
                            &durations,
                            &resource_requirements,
                            resource_capacity,
                        ))
                        .post()?;
                } else {
                    solver
                        .add_constraint(constraints::cumulative_decomposition(
                            start_times.to_vec(),
                            durations,
                            resource_requirements,
                            resource_capacity,
                        ))
                        .post()?;
                }
            }
            Constraint::Maximum { terms, rhs } => {
                let terms: Vec<_> = terms.into_iter().map(to_solver_variable).collect();
                let rhs = to_solver_variable(rhs);

                if use_global_propagator(Globals::Maximum) {
                    let _ = solver.add_constraint(constraints::maximum(terms, rhs)).post();
                } else {
                    let _ = solver.add_constraint(constraints::maximum_decomposition(terms, rhs)).post();
                }
            }
        }
    }

    Ok(())
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
    LinearLessEqual { terms: Vec<IntVariable>, rhs: i32 },
    Cumulative {
        start_times: Vec<IntVariable>,
        durations: Vec<u32>,
        resource_requirements: Vec<u32>,
        resource_capacity: u32,
    },
    Maximum { terms: Vec<IntVariable>, rhs: IntVariable },
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
            offset: self.offset * scale,
            id: self.id,
        }
    }

    pub fn offset(&self, offset: i32) -> IntVariable {
        IntVariable {
            scale: self.scale,
            offset: self.offset + offset,
            id: self.id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntVariableArray(usize);

impl IntVariableArray {
    pub fn as_array<'model>(
        &self,
        model: &'model Model,
    ) -> impl Iterator<Item = IntVariable> + 'model {
        let (_, range) = &model.arrays[self.0];

        (range.start..range.end).map(|id| IntVariable {
            scale: 1,
            offset: 0,
            id,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Output {
    Variable(IntVariable),
    Array(IntVariableArray),
}

#[derive(Clone, Debug)]
pub struct VariableMap {
    variables: Vec<AffineView<DomainId>>,
    names: Vec<String>,
    arrays: Vec<(String, Range<usize>)>,
}

impl VariableMap {
    pub fn to_solver_variable(&self, int_var: IntVariable) -> AffineView<DomainId> {
        self.variables[int_var.id]
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

    pub fn get_name(&self, output: &Output) -> String {
        match output {
            Output::Variable(int_var) => {
                let mut domain_name = self.names[int_var.id].clone();

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

            Output::Array(int_variable_array) => self.arrays[int_variable_array.0].0.clone(),
        }
    }

    pub fn get_array(&self, array: IntVariableArray) -> Vec<AffineView<DomainId>> {
        let (_, range) = &self.arrays[array.0];

        (range.start..range.end)
            .map(|idx| self.variables[idx].clone())
            .collect()
    }
}

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Globals {
    Circuit,
    Element,
    AllDifferent,
    Cumulative,
    Maximum,
}
