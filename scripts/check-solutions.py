#!/usr/bin/env python3
from argparse import ArgumentParser
from pathlib import Path
from dataclasses import dataclass
import json

from common import *


@dataclass
class Args:
    """Commandline arguments for this script."""

    experiment_dir: Path
    """The directory of the experiment."""


def run(args: Args):
    with (args.experiment_dir / "manifest.json").open('r') as manifest:
        context = Context.from_dict(json.load(manifest))

    check_runs(context)


if __name__ == "__main__":
    arg_parser = ArgumentParser(description="Check the solutions for an experiment.")

    arg_parser.add_argument("experiment_dir", type=Path, help="The directory containing the experiment data.")

    args = arg_parser.parse_args()

    run(Args(experiment_dir=args.experiment_dir))

