#!/usr/bin/env python3
import subprocess

from common import Args, ModelType, check_runs, evaluate

# Timeout per instance.
INSTANCE_TIMEOUT = 20

PROPAGATOR_MODELS: dict[str, ModelType] = {
    "dfs-circuit": "tsp",
    "forward-checking-circuit": "tsp",
    "all-different": "tsp",
    "element": "tsp",
    "time-table-cumulative": "rcpsp",
    "energetic-reasoning-cumulative": "rcpsp",
    "maximum": "rcpsp",
}

PROPAGATOR_TEST_MODULES = {
    "dfs-circuit": "circuit::dfs",
    "forward-checking-circuit": "circuit::forward_checking",
    "all-different": "all_different",
    "element": "element",
    "time-table-cumulative": "cumulative::time_table",
    "energetic-reasoning-cumulative": "cumulative::energetic_reasoning",
    "maximum": "maximum",
}

PROPAGATOR_GRADE_CONTRIBUTION = {
    "dfs-circuit": 8,
    "forward-checking-circuit": 4,
    "all-different": 10,
    "element": 7,
    "time-table-cumulative": 4,
    "energetic-reasoning-cumulative": 8,
    "maximum": 4,
}


def grade_propagator(propagator: str) -> int:
    """Grade a single global propagator. Return the contribution to the final grade for this propagator."""

    print(f"Grading {propagator}...")

    # For each propagator, run `cargo test`
    test_filter = f"tests::propagators::{PROPAGATOR_TEST_MODULES[propagator]}"
    result = subprocess.run(
        ["cargo", "test", test_filter],
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        print(result.stdout)
        return 0

    # For each propagator, run the associated model once with the propagator enabled. All solutions should be correct.
    context = evaluate(Args(
        model=PROPAGATOR_MODELS[propagator],
        timeout=INSTANCE_TIMEOUT,
        flags=["-G", propagator],
        allow_dirty=True,
    ))
    if context is None:
        return 0

    if not check_runs(context):
        return 0

    print(f"  Passes all tests!")
    return PROPAGATOR_GRADE_CONTRIBUTION[propagator]


def run():
    assert PROPAGATOR_MODELS.keys() == PROPAGATOR_TEST_MODULES.keys() and PROPAGATOR_MODELS.keys() == PROPAGATOR_GRADE_CONTRIBUTION.keys(), \
        "The keys for these dictionaries must be all the global names in the model executables."

    max_grade = sum(PROPAGATOR_GRADE_CONTRIBUTION.values())
    assert max_grade == 45, \
        f"Expected the maximum total grade to be 45 points. Was {max_grade}"

    propagators = PROPAGATOR_MODELS.keys()

    total_grade = 0

    for propagator in propagators:
        total_grade += grade_propagator(propagator)

    print(f"Grade = {total_grade}% / {max_grade}%")


if __name__ == "__main__":
    run()
