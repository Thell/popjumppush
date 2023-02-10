""" Implementation of the Koda Ruskey Ideals from Forest Posets Algorithm
    (As described in Knuth's TAOCP Volume 4A 7.2.11)
"""

from node_manipulation import group_indices_by_value


def koda_ruskey(active_nodes, focus_pointers, left_child, fringe_l, fringe_r):
    """ Koda-Ruskey Ideals of Forest Posets
    Knuth Algorithm K TAoCP 4A 7.2.1.1
    """

    while True:
        q = fringe_l[0]
        p = focus_pointers[q]
        focus_pointers[q] = q

        if not p:
            break

        if not active_nodes[p]:
            active_nodes[p] = 1
            if left_child[p]:
                q = fringe_r[p]
                fringe_l[q] = p - 1
                fringe_r[p - 1] = q
                fringe_r[p] = left_child[p]
                fringe_l[left_child[p]] = p
        else:
            active_nodes[p] = 0
            if left_child[p]:
                q = fringe_r[p - 1]
                fringe_r[p] = q
                fringe_l[q] = p

        focus_pointers[p] = focus_pointers[fringe_l[p]]
        focus_pointers[fringe_l[p]] = fringe_l[p]
        yield active_nodes


def visit(ideal, labels, output):
    """ Process/output ideals.
    """
    if output == 2:
        print(ideal)
    else:
        active_indices = [i for i, a in enumerate(ideal[1:]) if a]
        if output == 3:
            print(active_indices)
        else:
            result = [labels[i] for i in active_indices]
            result.sort()
            print(result)


def prep_args(root, parents, children):
    """ Return a tuple of the arguments for calling koda_ruskey.
    """
    _mapping, root, parents, children = sorted_post_order_indices(root, parents, children)
    left_child = leftmost_children_indices(root, parents, children)
    left_child = [parents.index(root) + 1] + left_child
    fringe_l, fringe_r = generate_sibling_arrays(parents, children)
    n = len(left_child) - 1
    active_nodes = [0] * (n + 1)
    focus_pointers = list(range(n + 1))  # 0..n
    fringe_l = [n] + fringe_l
    fringe_l[left_child[0]] = 0
    fringe_r = [left_child[0]] + fringe_r
    fringe_r[n] = 0

    return {
        "active_nodes": active_nodes,
        "focus_pointers": focus_pointers,
        "left_child": left_child,
        "fringe_l": fringe_l,
        "fringe_r": fringe_r
    }


def generate_sibling_arrays(parents, children):
    """ Create doubly linked list arrays for left <-> right siblings.
    """
    n = len(parents)
    left_siblings = [0] * n
    right_siblings = [0] * n
    child_indices = group_indices_by_value(parents)

    for _, children_indices in child_indices.items():
        for i, index in enumerate(children_indices):
            if i > 0:
                left_siblings[index] = children[children_indices[i - 1]]
            if i < len(children_indices) - 1:
                right_siblings[index] = children[children_indices[i + 1]]

    return left_siblings, right_siblings


def leftmost_children_indices(_root, parents, children):
    """ Return an array indicating the index of the left child of each parent.
    """
    return [parents.index(n) + 1 if n in parents else 0 for n in children]


def sorted_post_order_indices(root, parents, children):
    """ Reorganize given parents and children to be post order indexed values.
    """
    # Generate post order mappings.
    mapping = {}

    def post_order_traverse(node, index):
        for child in children:
            if parents[children.index(child)] == node:
                index = post_order_traverse(child, index)
        mapping[node] = index
        index += 1
        return index

    post_order_traverse(root, 1)

    # Re-label the parents and children arrays
    post_parents = [mapping[node] if node is not None else None for node in parents]
    post_children = [mapping[node] for node in children]

    # Sort by children indices.
    post_parents_children = list(zip(post_children, post_parents))
    post_parents_children.sort()
    post_children, post_parents = zip(*post_parents_children)

    return mapping, mapping.get(root), list(post_parents), list(post_children)
