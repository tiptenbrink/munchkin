from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Literal, Union
from subprocess import run
import shutil


MODELS = ["tsp", "rcpsp-makespan", "rcpsp-tardiness"]

ModelType = Union[Literal["tsp"], Literal["rcpsp-makespan"], Literal["rcpsp-tardiness"]] 

DATA_DIR = (Path(__file__).parent / ".." / "data").resolve()
EXPERIMENT_DIR = (Path(__file__).parent / ".." / "experiments").resolve()

INSTANCES = {
    "tsp": (DATA_DIR / "tsp"),
    "rcpsp-makespan": (DATA_DIR / "testing-rcpsp"),
    "rcpsp-tardiness": (DATA_DIR / "rcpsp"),
}

MINIZINC_MODELS = {
    "tsp": (Path(__file__).parent / ".." / "models" / "tsp.mzn").resolve(),
    "rcpsp-makespan": (Path(__file__).parent / ".." / "models" / "tsp.mzn").resolve(),
    "rcpsp-tardiness": (Path(__file__).parent / ".." / "models" / "tsp.mzn").resolve(),
}

SOLUTION_SEPARATOR = "-" * 10
OPTIMALITY_PROVEN = "=" * 10


@dataclass
class Context:
    """The context for the current evaluation."""

    directory: Path
    """The path of the current evaluation runs."""

    runs: Path
    """The directory containing the log files."""

    model: ModelType
    """The model being evaluated."""

    timeout: int
    """The timeout to give to each instance."""
    

    def to_dict(self) -> dict:
        """Turn the context into a JSON-serializable dictionary."""

        return {
            "directory": str(self.directory),
            "runs": str(self.runs),
            "model": self.model,
            "timeout": self.timeout,
        }


    @classmethod
    def from_dict(cls, dictionary: dict) -> 'Context':
        """Create a Context from a JSON-dictionary."""

        return cls(
            directory=Path(dictionary["directory"]),
            runs=Path(dictionary["runs"]),
            model=dictionary["model"],
            timeout=int(dictionary["timeout"]),
        )



def check_runs(context: Context):
    for run in context.runs.iterdir():
        check_run(run, context.model)


def check_run(run: Path, model: ModelType):
    instance_name = run.stem

    print(f"Checking {instance_name} for {model}")

    dzn_dir = generate_dzn_instances(run)

    model_path = MINIZINC_MODELS[model]
    data_path = INSTANCES[model] / f"{instance_name}.dzn"

    run_minizinc(model_path, data_path, dzn_dir)


def run_minizinc(model_path: Path, data_path: Path, solutions: Path):
    error_count = 0
    solution_count = 0

    for instance in solutions.iterdir():
        solution_count += 1
        result = run(["minizinc", model_path, data_path, instance], capture_output=True, text=True)

        if result.returncode != 0:
            print("STDOUT:")
            print(result.stdout)

            print("\nSTDERR:")
            print(result.stderr)

            raise Exception("MiniZinc failed.")

        if "UNSATISFIABLE" in result.stdout:
            error_count += 1
            print("Error detected in solution")
            print(f"  Solution: {instance.stem}")
            print(f"  Model: {model_path.stem}")
            print(f"  Data: {data_path.stem}")

    if error_count > 0:
        print(f"\n{error_count}/{solution_count} instances had errors.")
    elif solution_count > 0:
        print(f"All solutions are correct!")
    else:
        print(f"No reported solutions!")


def iter_solutions(run: Path) -> Iterable[str]:
    """Iterate over the individual solutions of a run."""

    output_log_path = run / "output.log"

    with output_log_path.open('r') as output:
        output = output.read()

    if "UNSATISFIABLE" in output or "UNKNOWN" in output:
        # There are no solutions in this file.
        return iter([])

    return filter(
        lambda s: s != "" and s != OPTIMALITY_PROVEN, 
        map(lambda s: s.strip(), output.split(SOLUTION_SEPARATOR))
    )


def generate_dzn_instances(run: Path) -> Path:
    """
    For all the reported solutions in this run, generate a DZN file.
    """

    solutions_dir = run / "solutions_dzn"
    if solutions_dir.is_dir():
        # We delete previous solutions if they are generated.
        shutil.rmtree(solutions_dir)

    solutions_dir.mkdir()

    solutions = list(iter_solutions(run))

    print(f"  Identified {len(solutions)} solution(s).")

    for idx, solution in enumerate(solutions):
        solution = solution.strip()

        if solution == "":
            continue

        solution_file = solutions_dir / f"sol-{idx}.dzn"

        with solution_file.open('w') as solution_file:
            for line in solution.split("\n"):
                # Disregard lines (aka variables) that start with an '_' as they are not 
                # variables that correspond to the MZN model.
                if line.startswith("_"):
                    continue

                solution_file.write(f"{line}\n")

    return solutions_dir
