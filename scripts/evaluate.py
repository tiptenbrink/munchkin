#!/usr/bin/env python3

from argparse import ArgumentParser, REMAINDER

from common import *


if __name__ == "__main__":
    arg_parser = ArgumentParser(description="Evaluate a Munckin model")
     
    arg_parser.add_argument(
         "--allow-dirty", 
         action="store_true", 
         help="Allow uncommitted files when running the experiment."
    )

    arg_parser.add_argument("model", help="The model to evaluate.", choices=MODELS)
    arg_parser.add_argument("timeout", help="Time budget for every instance in seconds.", type=int)

    arg_parser.add_argument("model_flags", nargs=REMAINDER, help="Arguments after --")

    args = arg_parser.parse_args()

    evaluate(Args(
        model=args.model,
        timeout=args.timeout,
        flags=args.model_flags,
        allow_dirty=args.allow_dirty,
    ))
