use std::any::Any;
use std::path::PathBuf;
use std::time::Duration;

use clap::ValueEnum;

use crate::branching::Brancher;
use crate::model::Globals;
use crate::model::IntVariable;
use crate::model::Model;
use crate::model::VariableMap;
use crate::options::SolverOptions;
use crate::results::ProblemSolution;
use crate::results::SatisfactionResult;
use crate::termination::TimeBudget;
use crate::Solver;

pub trait OptionEnum: ValueEnum + Clone + Send + Sync + Any + Default {}
impl<T> OptionEnum for T where T: ValueEnum + Clone + Send + Sync + Any + Default {}

#[derive(Debug, clap::Parser)]
pub struct Cli<SearchStrategies: OptionEnum> {
    /// The data for the model.
    pub instance: PathBuf,

    #[command(subcommand)]
    pub command: Action<SearchStrategies>,
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum Action<SearchStrategies: OptionEnum> {
    /// Solve the given instance.
    Solve {
        /// The constraints that should _not_ be decomposed.
        ///
        /// Multiple constraints can be provided by passing this option multiple times.
        #[arg(short = 'G', long = "global")]
        globals: Vec<Globals>,

        /// The file path to which the proof will be written.
        ///
        /// If no path is provided, a proof will not be produced.
        #[arg(short = 'P')]
        proof_path: Option<PathBuf>,

        /// The search strategy to use.
        #[arg(short = 'S', long = "search", value_enum, default_value_t)]
        search_strategy: SearchStrategies,

        /// The number of seconds the solver is allowed to run.
        time_out: u64,
    },

    /// Check the proof of this instance.
    Verify {
        /// The file path to the proof.
        proof_path: PathBuf,
    },
}

/// Definition of a problem instance to be solved with Munchkin.
pub trait Problem<SearchStrategies>: Sized {
    /// Constructor function which creates an instance of `Self`, as well as the [`Model`] for the
    /// problem.
    fn create(data: dzn_rs::DataFile<i32>) -> anyhow::Result<(Self, Model)>;

    fn get_search(
        &self,
        strategy: SearchStrategies,
        solver: &Solver,
        solver_variables: &VariableMap,
    ) -> impl Brancher + 'static;

    fn get_output_variables(&self) -> impl Iterator<Item = IntVariable> + '_;
}

#[macro_export]
macro_rules! entry_point {
    (problem = $problem:ident, search_strategies = $search_strategies:ident) => {
        fn main() -> anyhow::Result<()> {
            $crate::runner::run::<$problem, $search_strategies>()
        }
    };
}

pub fn run<ProblemType, SearchStrategies>() -> anyhow::Result<()>
where
    ProblemType: Problem<SearchStrategies>,
    SearchStrategies: OptionEnum,
{
    use anyhow::Context;
    use clap::Parser;

    let args = Cli::<SearchStrategies>::parse();

    let data = std::fs::read_to_string(&args.instance)
        .with_context(|| format!("Error reading {}", args.instance.display()))?;

    let data = dzn_rs::parse::<i32>(data.as_bytes())
        .with_context(|| format!("Failed to parse DZN from {}", args.instance.display()))?;

    let (instance, model) = ProblemType::create(data)?;

    match args.command {
        Action::Solve {
            globals,
            proof_path,
            search_strategy,
            time_out,
        } => solve(
            model,
            instance,
            search_strategy,
            globals,
            proof_path,
            Duration::from_secs(time_out),
        ),
        Action::Verify { proof_path } => verify(model, proof_path),
    }
}

pub fn solve<SearchStrategies>(
    model: Model,
    instance: impl Problem<SearchStrategies>,
    search_strategy: SearchStrategies,
    globals: Vec<Globals>,
    _proof_path: Option<PathBuf>,
    time_out: Duration,
) -> anyhow::Result<()> {
    let (mut solver, solver_variables) = model.into_solver(
        SolverOptions {
            ..Default::default()
        },
        |global| globals.contains(&global),
    );

    let mut brancher = instance.get_search(search_strategy, &solver, &solver_variables);

    let result = solver.satisfy(&mut brancher, &mut TimeBudget::starting_now(time_out));

    match result {
        SatisfactionResult::Satisfiable(solution) => {
            println!("SATISFIABLE");

            let variables = instance.get_output_variables();

            for model_variable in variables {
                let solver_variable = solver_variables.to_solver_variable(model_variable);

                let name = solver_variables.get_name(model_variable);

                println!(
                    "{name} = {}",
                    solution.get_integer_value(solver_variable.clone())
                );
            }
        }
        SatisfactionResult::Unsatisfiable => println!("UNSATISFIABLE"),
        SatisfactionResult::Unknown => println!("UNKNOWN"),
    }

    Ok(())
}

pub fn verify(model: Model, proof_path: PathBuf) -> anyhow::Result<()> {
    todo!()
}
