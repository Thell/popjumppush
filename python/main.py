""" Main Pop Jump Push and Koda Ruskey functions.
Functions to benchmark, summarize, or output the ideals for test sets.
"""

import argparse
from argparse import RawTextHelpFormatter
import sys

from typing import Final  # for high resolution timing
import time  # used for timing specific functions when needed.
import copy  # used for the args in benchmarks

from sample_data import get_sample_data
from sample_data import summarize_sample_data

from node_manipulation import count_subtrees
from node_manipulation import get_sorted_children
from node_manipulation import postorder_to_preorder

from pop_jump_push import prep_args as prep_pop_jump_push_args
from pop_jump_push import pop_jump_push

from koda_ruskey import prep_args as prep_koda_ruskey_args
from koda_ruskey import koda_ruskey

INITIAL_TIMESTAMP: Final[float] = time.time()
INITIAL_TIMESTAMP_PERF_COUNTER: Final[float] = time.perf_counter()


def get_timestamp_float() -> float:
    """High resolution timestamp.
    """
    dt_sec = time.perf_counter() - INITIAL_TIMESTAMP_PERF_COUNTER
    return INITIAL_TIMESTAMP + dt_sec


def prep_algo_args(algo, root, parents, children):
    """ Return the arguments for a generator as a set.
    """
    if algo == "pop_jump_push":
        return prep_pop_jump_push_args(root, parents, children)
    return prep_koda_ruskey_args(root, parents, children)


def dump_args(algos, root, parents, children):
    """ Print algorithm arguments.
    """
    print("\n=== Test Set Data ===")
    arg = "root"
    print(f"{arg:>18}: {root}")
    arg = "parents"
    print(f"{arg:>18}: {parents}")
    arg = "children"
    print(f"{arg:>18}: {children}\n")

    for algo in algos:
        print(f"=== {algo} ===")
        args = prep_algo_args(algo, root, parents, children)
        for key, value in args.items():
            print(f"{key:>18}: {value}")
            continue
        print()


def get_generator(algo, args):
    """ Return the desired generator.

        args: should be a set, the values will be spread '*args.values()'
    """
    if algo == "pop_jump_push":
        return pop_jump_push(*args.values())
    return koda_ruskey(*args.values())


def benchmark(algos, root, parents, children, reps):
    """ Time the generation of all ideals/subtrees.
    """
    ideals_count = count_subtrees(root, parents, children) - 1
    print(f"Generating {ideals_count:,} ideals from {len(children)} nodes {reps} times.\n")

    for algo in algos:
        algo_reps = reps
        print(f"=== {algo} ===")

        args = prep_algo_args(algo, root, parents, children)
        start_time = get_timestamp_float()
        time_delta = 10e10

        i = 0
        while algo_reps:
            algo_reps -= 1
            generator = get_generator(algo, copy.deepcopy(args))

            run_start_time = get_timestamp_float()
            # We have no need to actually visit the ideal to get our timings for generation.
            for _ideal in generator:
                i += 1
            run_time_delta = get_timestamp_float() - run_start_time

            time_delta = min(time_delta, run_time_delta)

        end_time_delta = get_timestamp_float() - start_time

        print(
            f"\n\tCompleted processing {i} ideals from {reps} trees with {ideals_count} ideals.",
            f"\n\tAvg Duration per tree {end_time_delta / reps}"
            f"\n\tBest Duration per tree {time_delta}\n\t"
            f"{(end_time_delta / reps) / ideals_count * 10e9} ns avg per ideal"
            f"\n\t{time_delta / ideals_count * 10e9} ns best per ideal\n")


def generate_ideals(algos, root, parents, children, output):
    """ Output generated ideals.
    Output types:
        - 2 raw algorithm output
        - 3 ideals node output in native algorithm order
        - 4 ideals node output from native to sorted preorder (for comparing output)
    """
    for algo in algos:
        print(f"=== {algo} ===")

        args = prep_algo_args(algo, root, parents, children)
        generator = get_generator(algo, args)

        sorted_children = get_sorted_children(algo, root, parents, children)
        print(f"ordered children: {sorted_children}")

        if algo == "pop_jump_push" or output == 4:
            print("preorder ideals...")
        else:
            print("postorder ideals...")

        for ideal in generator:
            if output == 2:
                print(ideal)
            elif output == 3:
                if algo == "koda_ruskey":
                    print([sorted_children[i] for i, a in enumerate(ideal[1:]) if a])
                else:
                    print(ideal)
            else:
                if algo == "koda_ruskey":
                    result = [sorted_children[i] for i, a in enumerate(ideal[1:]) if a]
                    result = postorder_to_preorder(result)
                    result.sort()
                    print(result)
                else:
                    print([sorted_children[i] for i in ideal])


def main(algos, node_data, output, reps):
    """ main dispatch.
    """
    root, parents, children = node_data
    if output == 0:
        benchmark(algos, root, parents, children, reps)
    elif output == 1:
        dump_args(algos, root, parents, children)
    else:
        generate_ideals(algos, root, parents, children, output)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Main Pop Jump Push and Koda Ruskey functions.",
                                     formatter_class=RawTextHelpFormatter)
    parser.add_argument("-a",
                        "--algo",
                        type=int,
                        choices=[0, 1, 2],
                        required=True,
                        help="""0 = Pop Jump Push
1 = Koda Ruskey
2 = both
""")
    parser.add_argument("-s",
                        "--sample_set",
                        type=str,
                        required=True,
                        help="The name of the sample data to load from sample_data.")
    parser.add_argument("-o",
                        "--output",
                        type=int,
                        choices=[0, 1, 2, 3, 4],
                        required=True,
                        help="""0 performance timing
1 algorithm arguments dump
2 raw algorithm output
3 ideals node output in native algorithm order
4 ideals node output from native to sorted preorder (for comparing output)
""")
    parser.add_argument(
        "-r",
        "--reps",
        type=int,
        required=True,
        help="The number of times to perform the performance test. (REPS 0 produces summary)")

    cli_args = parser.parse_args()

    ALGO = cli_args.algo
    SAMPLE_SET = cli_args.sample_set
    OUTPUT = cli_args.output
    REPS = cli_args.reps

    _node_data = get_sample_data(SAMPLE_SET)

    if REPS == 0:
        summarize_sample_data(*_node_data)
        sys.exit()

    _algos = []
    if ALGO == 0:
        _algos = ["pop_jump_push"]
    elif ALGO == 1:
        _algos = ["koda_ruskey"]
    else:
        _algos = ["pop_jump_push", "koda_ruskey"]

    main(_algos, _node_data, OUTPUT, REPS)

    sys.exit()

# if __name__ == '__main__':
#     ALGO = int(sys.argv[1])
#     SAMPLE_SET = sys.argv[2]
#     OUTPUT = int(sys.argv[3])
#     REPS = int(sys.argv[4])

#     _node_data = get_sample_data(SAMPLE_SET)

#     if REPS == 0:
#         summarize_sample_data(*_node_data)
#         sys.exit()

#     _algos = []
#     if ALGO == 0:
#         _algos = ["pop_jump_push"]
#     elif ALGO == 1:
#         _algos = ["koda_ruskey"]
#     else:
#         _algos = ["pop_jump_push", "koda_ruskey"]

#     main(_algos, _node_data, OUTPUT, REPS)

#     sys.exit()
