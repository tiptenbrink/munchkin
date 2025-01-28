#!/usr/bin/env python3

from argparse import ArgumentParser
from dataclasses import dataclass
from pathlib import Path
from subprocess import run
from datetime import datetime
import json
import sys

from common import *



@dataclass
class Args:
    """The arguments passed to the evaluation script."""

    model: ModelType
    """The model to evaluate."""

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

    context = Context(
        directory=directory, 
        runs=runs,
        model=args.model,
        timeout=args.timeout,
    )

    with (directory / "manifest.json").open('w') as context_file:
        json.dump(context.to_dict(), context_file, indent=4)

    return context


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


def evaluate(args: Args):
    context = initialise(args)

    run_instances(context)
    check_runs(context)



if __name__ == "__main__":
    arg_parser = ArgumentParser(description="Evaluate a Munckin model")

    arg_parser.add_argument("model", help="The model to evaluate.", choices=MODELS)
    arg_parser.add_argument("timeout", help="Time budget for every instance in seconds.", type=int)

    args = arg_parser.parse_args()

    evaluate(Args(
        model=args.model,
        timeout=args.timeout,
    ))
