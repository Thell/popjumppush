""" Implementation of the Pop Jump Push Algorithm for generating all ideals of an arborescence.
"""

from node_manipulation import sorted_traversal_pre_order


def pop_jump_push(num_nodes, sequence_indices, jump_indices):
    """ Given a tree rooted at r generate all rooted subtrees rooted at r by indices.
    """
    while sequence_indices:
        yield sequence_indices
        index = jump_indices[sequence_indices.pop()]
        while index < num_nodes:
            sequence_indices.append(index)
            index += 1


def visit(ideal, labels, output):
    """ Process/output ideals.
    """
    if output == 2:
        print(ideal)
    elif output == 3:
        result = [labels[i] for i in ideal]
        print(result)
    else:
        result = [labels[i] for i in ideal]
        result.sort()
        print(result)


def prep_args(root, parents, children):
    """ Return a tuple of the arguments for calling pop_jump_push.
    """
    num_nodes = len(children)
    parents, children = sorted_traversal_pre_order(root, parents, children)
    sequence_indices = list(range(num_nodes))
    jump_indices = generate_jump_indices(parents, children)

    return {
        "num_nodes": num_nodes,
        "sequence_indices": sequence_indices,
        "jump_indices": jump_indices
    }


def generate_jump_indices(parents, children):
    """ - Return indices of the first node of the next right subtree for each node.

    - This is a one past last value; range(i, i_end) aka [i..i_end) covers all nodes in the subtree.

    - Children and parents must be in traversal pre-order!
    """
    num_nodes = len(children)
    jump_indices = [0] * num_nodes
    parent_indices = [0] + [children.index(p) for p in parents[1:]]
    for index in reversed(range(num_nodes)):
        jump_indices[index] = max(index + 1, jump_indices[index])
        jump_indices[parent_indices[index]] = max(jump_indices[index],
                                                  jump_indices[parent_indices[index]])
    return jump_indices
