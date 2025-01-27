#!/usr/bin/env python3

from argparse import ArgumentParser
from typing import Literal, Union
from dataclasses import dataclass
from pathlib import Path
from subprocess import run
from datetime import datetime
import sys
import shutil


MODELS = ["tsp", "rcpsp-makespan", "rcpsp-tardiness"]

ModelType = Union[Literal["tsp"], Literal["rcpsp-makespan"], Literal["rcpsp-tardiness"]] 

DATA_DIR = (Path(__file__).parent / ".." / "data").resolve()
EXPERIMENT_DIR = (Path(__file__).parent / ".." / "experiments").resolve()

INSTANCES = {
    "tsp": (DATA_DIR / "tsp"),
    "rcpsp-makespan": (DATA_DIR / "rcpsp"),
    "rcpsp-tardiness": (DATA_DIR / "rcpsp"),
}

MINIZINC_MODELS = {
    "tsp": (Path(__file__).parent / ".." / "models" / "tsp.mzn").resolve(),
    "rcpsp-makespan": (Path(__file__).parent / ".." / "models" / "tsp.mzn").resolve(),
    "rcpsp-tardiness": (Path(__file__).parent / ".." / "models" / "tsp.mzn").resolve(),
}

SOLUTION_SEPARATOR = "-" * 10

@dataclass
class Args:
    """The arguments passed to the evaluation script."""

    model: ModelType
    """The model to evaluate."""

    timeout: int
    """The timeout to give to each instance."""


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
    


def initialise(args: Args) -> Context:
    """
    Prepare everything for evaluation.
    """

    # Create the directory which stores all the logs, if it does not exist.
    EXPERIMENT_DIR.mkdir(exist_ok=True)

    # Create a directory for the current evaluation.
    timestamp = datetime.now().strftime("%Y%m%d-%H.%M.%S.%f")
    directory = EXPERIMENT_DIR / f"{timestamp}-{args.model}"

    try:
        directory.mkdir()
    except FileExistsError:
        print("Error creating evaluation context directory. Please try again.")
        sys.exit(1)

    runs = directory / "runs"
    runs.mkdir()

    return Context(
        directory=directory, 
        runs=runs,
        model=args.model,
        timeout=args.timeout,
    )


def run_instances(context: Context):
    instances = INSTANCES[context.model]

    for instance in instances.iterdir():
        run_instance(instance, context)


def run_instance(instance: Path, context: Context):
    print(f"Evaluating instance {instance.stem}")

    instance_directory = context.runs / instance.stem
    instance_directory.mkdir()

    log_file_path = instance_directory / "output.log"
    err_file_path = instance_directory / "output.err"

    with log_file_path.open('w') as log_file:
        with err_file_path.open('w') as err_file:
            run(
                ["cargo", "run", "--example", context.model, "--", instance, "solve", str(context.timeout)],
                stdout=log_file,
                stderr=err_file,
            ) 


def check_runs(runs: Path, model: ModelType):
    for run in runs.iterdir():
        check_run(run, model)


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

        if "UNSATISFIABLE" in result.stdout:
            error_count += 1
            print(f"Error detected when checking solution {instance.stem} for model {model_path} and data file {data_path}.")

    print(f"{error_count}/{solution_count} instances had errors.")


def generate_dzn_instances(run: Path) -> Path:
    """
    For all the reported solutions in this run, generate a DZN file.
    """

    solutions_dir = run / "solutions_dzn"
    if solutions_dir.is_dir():
        # We delete previous solutions if they are generated.
        shutil.rmtree(solutions_dir)

    solutions_dir.mkdir()

    output_log_path = run / "output.log"

    with output_log_path.open('r') as output:
        output = output.read()

    if "UNSATISFIABLE" in output or "UNKNOWN" in output:
        # There are no solutions in this file.
        return

    solutions = list(
        filter(
            lambda s: s != "", 
            map(lambda s: s.strip(), output.split(SOLUTION_SEPARATOR))
        )
    )

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


def evaluate(args: Args):
    context = initialise(args)

    run_instances(context)
    check_runs(context.runs, context.model)



if __name__ == "__main__":
    arg_parser = ArgumentParser(description="Evaluate a Munckin model")

    arg_parser.add_argument("model", help="The model to evaluate.", choices=MODELS)
    arg_parser.add_argument("timeout", help="Time budget for every instance in seconds.", type=int)

    args = arg_parser.parse_args()

    evaluate(Args(
        model=args.model,
        timeout=args.timeout,
    ))
