use std::collections::HashMap;
use std::collections::VecDeque;

pub(crate) fn count_subtrees(root: usize, parents: &[usize], children: &Vec<usize>) -> usize {
    /*!  - Return the total number of possible subtrees rooted at root.

    Each leaf can be either active or inactive (for two states) and each parent is the product
    of its children's subtree counts plus 1.
    */
    let child_indices = group_indices_by_value(parents);
    let num_nodes = children.len();
    let root_index = children.iter().position(|&n| n == root).unwrap();

    let mut stack = VecDeque::new();
    let mut nodes = Vec::new();
    stack.push_back(root_index);

    while let Some(node) = stack.pop_back() {
        nodes.push(node);
        for child_index in child_indices
            .get(&children[node])
            .unwrap_or(&vec![])
            .iter()
            .rev()
        {
            stack.push_back(*child_index);
        }
    }

    let mut subtree_counts = vec![1; num_nodes];
    for &node in nodes.iter().rev() {
        let mut count = subtree_counts[node];
        if let Some(node_children) = child_indices.get(&children[node]) {
            for &child_index in node_children {
                let child_counts = subtree_counts[child_index];
                count *= child_counts;
            }
        }
        count += 1;
        subtree_counts[node] = count;
    }

    subtree_counts[0] - 1
}

pub(crate) fn group_indices_by_value(values: &[usize]) -> HashMap<usize, Vec<usize>> {
    /*!  - Returns HashMap keyed by unique values with occurance indices as the values.
     */
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, &value) in values.iter().enumerate() {
        groups.entry(value).or_default().push(index);
    }
    groups
}

pub(crate) fn arrange_by_traversal_pre_order(
    root: usize,
    parents: &[usize],
    children: &[usize],
) -> (Vec<usize>, Vec<usize>) {
    /*!  - Returns children and parents in traversal pre-order. */
    let mut result_parent = vec![];
    let mut result_child = vec![];
    let mut stack = vec![(root, None)];
    let child_indices = group_indices_by_value(parents);

    while let Some((node, parent)) = stack.pop() {
        result_child.push(node);
        result_parent.push(parent.unwrap_or(0));
        if let Some(child_indices) = child_indices.get(&node) {
            for &child_index in child_indices.iter().rev() {
                stack.push((children[child_index], Some(node)));
            }
        }
    }
    (result_parent, result_child)
}

pub(crate) fn arrange_by_traversal_post_order(
    root: usize,
    parents: &[usize],
    children: &[usize],
) -> (Vec<usize>, Vec<usize>) {
    /*!  - Returns children and parents in traversal post-order. */
    let mut result_parent = vec![];
    let mut result_child = vec![];
    let mut stack = vec![(root, None)];
    let child_indices = group_indices_by_value(parents);

    while let Some((node, parent)) = stack.pop() {
        if let Some(child_indices) = child_indices.get(&node) {
            for &child_index in child_indices.iter() {
                stack.push((children[child_index], Some(node)));
            }
        }
        result_child.push(node);
        result_parent.push(parent.unwrap_or(0));
    }
    result_parent.reverse();
    result_child.reverse();
    (result_parent, result_child)
}

pub(crate) fn arrange_largest_subtrees(
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    left: bool,
) -> (Vec<usize>, Vec<usize>) {
    /*!  - Arranges subtrees of a rooted tree by size.*/
    let mut result_parents = vec![];
    let mut result_children = vec![];
    let mut stack = vec![(root, 0)];
    let mut stack2 = vec![(root, 0)];
    let child_indices = group_indices_by_value(parents);

    let cmp = |a: &usize, b: &usize| {
        let mut a_count = count_subtrees_at(children[*a], &child_indices, children);
        let b_count = count_subtrees_at(children[*b], &child_indices, children);
        if a_count == b_count {
            a_count += 1;
            b_count.cmp(&a_count)
        } else if left {
            b_count.cmp(&a_count)
        } else {
            a_count.cmp(&b_count)
        }
    };

    while let Some((node, parent)) = stack.pop() {
        let mut node_children = match child_indices.get(&node) {
            Some(c) => c.clone(),
            None => vec![],
        };

        node_children.sort_by(cmp);

        for child in node_children.iter() {
            stack.push((children[*child], node));
            stack2.push((children[*child], node));
        }

        result_parents.push(parent);
        result_children.push(node);
    }
    (result_parents, result_children)
}

pub(crate) fn count_subtrees_at(
    root: usize,
    child_indices: &HashMap<usize, Vec<usize>>,
    children: &Vec<usize>,
) -> usize {
    /*!  - Returns the number of subtrees rooted at the given node.*/
    let mut count = 1;
    if let Some(c) = child_indices.get(&root) {
        for child in c {
            count *= count_subtrees_at(children[*child], child_indices, children);
        }
    }
    count + 1
}