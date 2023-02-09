""" Sample tree data for ideals generation.

Assumes root is children[0]
"""

import sys
from datetime import timedelta  # for sample_data_summary time estimation.
from node_manipulation import count_subtrees  # for sample_data_summary

TEST_SETS = {
    "set_Alpha": {
        "parents": [None, "A", "B", "A", "D", "D", "F", "F"],
        "children": ["A", "B", "C", "D", "E", "F", "G", "H"],
    },
    "set_7Readme": {
        "parents": [None, 1, 1, 1, 2, 2, 3],
        "children": [1, 2, 3, 4, 5, 6, 7],
    },
    "set_Ruskey": {
        "parents": [None, 1, 2, 1, 4, 4, 6, 6],
        "children": [1, 2, 3, 4, 5, 6, 7, 8],
    },
    "set_13M": {
        "parents": [None, 1, 2, 3, 4, 5, 5, 1, 8, 9, 8, 11, 11],
        "children": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
    },
    "set_3B": {
        "parents": [None, 1, 1],
        "children": [1, 2, 3],
    },
    "set_3W": {
        "parents": [None, 1, 1],
        "children": [1, 2, 3],
    },
    "set_3D": {
        "parents": [None, 1, 2],
        "children": [1, 2, 3],
    },
    "set_7B": {
        "parents": [None, 1, 2, 2, 1, 5, 5],
        "children": [1, 2, 3, 4, 5, 6, 7],
    },
    "set_7W": {
        "parents": [None, 1, 1, 1, 1, 1, 1],
        "children": [1, 2, 3, 4, 5, 6, 7],
    },
    "set_7D": {
        "parents": [None, 1, 2, 3, 4, 5, 6],
        "children": [1, 2, 3, 4, 5, 6, 7],
    },
    "set_15B": {
        "parents": [None, 1, 2, 3, 3, 2, 6, 6, 1, 9, 10, 10, 9, 13, 13],
        "children": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    },
    "set_15W": {
        "parents": [None, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        "children": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    },
    "set_15D": {
        "parents": [None, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        "children": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    },
    "set_31B": {
        "parents": [
            None, 1, 2, 3, 4, 4, 3, 7, 7, 2, 10, 11, 11, 10, 14, 14, 1, 17, 18, 19, 19, 18, 22, 22,
            17, 25, 26, 26, 25, 29, 29
        ],
        "children": [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31
        ],
    },
    "set_31W": {
        "parents": [
            None, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1
        ],
        "children": [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31
        ],
    },
    "set_31D": {
        "parents": [
            None, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30
        ],
        "children": [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31
        ],
    },
    "set_53X": {
        "parents": [
            None, 1, 2, 3, 4, 5, 5, 4, 8, 8, 3, 11, 12, 12, 11, 15, 15, 2, 18, 19, 20, 20, 19, 23,
            23, 18, 26, 27, 27, 26, 30, 30, 1, 33, 34, 35, 36, 36, 35, 39, 39, 34, 42, 43, 43, 42,
            46, 46, 33, 49, 50, 51, 51
        ],
        "children": [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53
        ],
    },
    "set_63B": {
        "parents": [
            None, 1, 2, 3, 4, 5, 5, 4, 8, 8, 3, 11, 12, 12, 11, 15, 15, 1, 18, 19, 20, 20, 19, 23,
            23, 18, 26, 27, 27, 26, 30, 30, 1, 33, 34, 35, 36, 36, 35, 39, 39, 34, 42, 43, 43, 42,
            46, 46, 33, 49, 50, 51, 51, 50, 54, 54, 49, 57, 58, 58, 57, 61, 61
        ],
        "children": [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
        ],
    },
}


def summarize_sample_data(root, parents, children):
    """ Generate summary of sample data.
    """

    expected_subtree_count = count_subtrees(root, parents, children)
    per_ideal_ns = 72
    rate = round(1 / (per_ideal_ns * 10E-10))
    estimated_completion_time = expected_subtree_count / rate
    estimated_completion_time_years = estimated_completion_time // 31_536_000
    estimated_completion_time -= estimated_completion_time_years * 31_536_000
    estimated_completion_time = timedelta(seconds=estimated_completion_time)

    print(
        f"num_nodes: {len(children)}", f"\nsubtrees count: {expected_subtree_count:,}",
        f"\nestimated time @ {per_ideal_ns}(ns) per ideal ({rate:,}/s):"
        f" {round(estimated_completion_time_years):,} years", f"and {estimated_completion_time}")
    return 0


def get_sample_data(sample_set):
    """Get static test data.

    Returns root, parents and children tuple.
    """

    print("Getting test set:", sample_set)
    try:
        test_set = TEST_SETS[sample_set]
    except KeyError:
        print("Invalid set name!\nValid sets are:")
        print(TEST_SETS.keys())
        sys.exit()
    else:
        parents = test_set["parents"]
        children = test_set["children"]
        root = children[0]

        return (root, parents, children)
