use clap::ValueEnum;
use dzn_rs::DataFile;
use munchkin::branching::branchers::independent_variable_value_brancher::IndependentVariableValueBrancher;
use munchkin::branching::Brancher;
use munchkin::branching::InDomainMin;
use munchkin::branching::InputOrder;
use munchkin::model::IntVariable;
use munchkin::model::Model;
use munchkin::model::VariableMap;
use munchkin::runner::Problem;
use munchkin::Solver;

munchkin::entry_point!(problem = Rcpsp, search_strategies = SearchStrategies);

#[derive(Clone, Copy, Default, ValueEnum)]
enum SearchStrategies {
    #[default]
    Default,
}

struct Rcpsp {
    start_times: Vec<IntVariable>,
}

impl Problem<SearchStrategies> for Rcpsp {
    fn create(data: DataFile<i32>) -> anyhow::Result<(Self, Model)> {
        todo!()
    }

    fn get_search(
        &self,
        strategy: SearchStrategies,
        _: &Solver,
        solver_variables: &VariableMap,
    ) -> impl Brancher + 'static {
        match strategy {
            SearchStrategies::Default => IndependentVariableValueBrancher::new(
                InputOrder::new(
                    solver_variables
                        .to_solver_variables(self.start_times.clone())
                        .collect(),
                ),
                InDomainMin,
            ),
        }
    }

    fn get_output_variables(&self) -> impl Iterator<Item = IntVariable> + '_ {
        self.start_times.iter().copied()
    }
}
