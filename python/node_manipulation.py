""" Node array manipulation functions to prepare or finalize for ideals generation/reporting.
"""

from collections import defaultdict  # for grouped indices


def count_subtrees(root, parents, children):
    """ Return the count of subtrees rooted at r in a tree rooted at r
    """
    child_indices = group_indices_by_value(parents)

    def traverse(node):
        count = 1
        _ = [count := count * traverse(children[child]) for child in child_indices.get(node, [])]
        count += 1
        return count

    count = traverse(root) - 1
    return count


def get_sorted_children(algo, root, parents, children):
    """ Return the children in either preorder or postorder.
    """
    sorted_children = []
    if algo == "pop_jump_push":
        _, sorted_children = sorted_traversal_pre_order(root, parents, children)
    else:
        _, sorted_children = sorted_traversal_post_order(root, parents, children)
    return sorted_children


def group_indices_by_value(values):
    """ Return dict keyed by parent with value of indices of parent.
    """
    grouped_indices = defaultdict(list)
    for index, value in enumerate(values):
        grouped_indices[value].append(index)
    return grouped_indices


def postorder_to_preorder(postorder):
    """ Convert the postorder indices to preorder.
    """
    if not postorder:
        return []

    root = postorder[-1]
    idx = 0
    for i in range(len(postorder) - 1):
        if postorder[i] > root:
            idx = i
            break

    return [root] + postorder_to_preorder(postorder[:idx]) \
                  + postorder_to_preorder(postorder[idx:-1])


def sorted_traversal_pre_order(root, parents, children):
    """ Return children and parents in traversal pre-order from root.
    """
    child_indices = group_indices_by_value(parents)

    def traverse(child, parent):
        result_children.append(child)
        result_parents.append(parent)
        for child_index in child_indices.get(child, []):
            traverse(children[child_index], child)

    result_parents = []
    result_children = []
    traverse(root, None)
    return result_parents, result_children


def sorted_traversal_post_order(root, parents, children):
    """ Return children and parents in traversal pre-order from root.
    """
    child_indices = group_indices_by_value(parents)

    def traverse(child, parent):
        for child_index in child_indices.get(child, []):
            traverse(children[child_index], child)
        result_children.append(child)
        result_parents.append(parent)

    result_parents = []
    result_children = []
    traverse(root, None)
    return result_parents, result_children
