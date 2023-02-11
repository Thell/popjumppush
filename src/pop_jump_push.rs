//! # Implementation of the Pop Jump Push Algorithm for generating all ideals of an arborescence.
//!

use std::time::Instant;

use crate::node_manipulation::arrange_by_traversal_pre_order;
use crate::node_manipulation::count_subtrees;

fn pop_jump_push(
    num_nodes: usize,
    sequence_indices: &mut Vec<usize>,
    jump_indices: &[usize],
    labels: &[usize],
    output: u8,
) {
    /*!  - Implements the Pop Jump Push algorithm. */
    while !sequence_indices.is_empty() {
        visit(sequence_indices, labels, output);
        let index = jump_indices[sequence_indices.pop().unwrap()];
        if index < num_nodes {
            sequence_indices.extend(index..num_nodes);
        }
    }
}

pub(crate) fn visit(ideal: &[usize], labels: &[usize], output: u8) {
    /*!  -  Process/output ideals. */
    // This just ensures the compiler doesn't optimize anything away during `output == 0` benchmarking.
    let ideal = std::hint::black_box(ideal);

    if output == 2 {
        println!("{ideal:?}")
    } else if output >= 3 {
        let mut result: Vec<_> = ideal.iter().map(|i| labels[*i]).collect();
        result.sort();
        println!("{result:?}");
    };
}

pub(crate) fn prep_args(
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    output: u8,
) -> (usize, Vec<usize>, Vec<usize>, Vec<usize>) {
    /*!  -  Return a tuple of the arguments for calling pop_jump_push. */
    let num_nodes = children.len();
    let sequence_indices = (0..num_nodes).collect::<Vec<_>>();
    let (parents, children) = arrange_by_traversal_pre_order(root, parents, children);
    let jump_indices = generate_jump_indices(&parents, &children);
    if output == 1 {
        let arg = "num_nodes";
        println!("{arg:>18}: {num_nodes}");
        let arg = "sequence_indices";
        println!("{arg:>18}: {sequence_indices:?}");
        let arg = "jump_indices";
        println!("{arg:>18}: {jump_indices:?}");
    }
    (num_nodes, sequence_indices, jump_indices, children)
}

pub(crate) fn generate_jump_indices(parents: &[usize], children: &[usize]) -> Vec<usize> {
    /*!  - Returns the pre-order traversal end indices for the subtree rooted at each node.

    This is a one past last value; range(i, i_end) aka [i..i_end) covers all nodes in the subtree.

    Children and parents must be in traversal pre-order!
    */
    let num_nodes = children.len();
    let mut end_indices = vec![0; num_nodes];
    let mut parent_indices = vec![0];

    for p in parents[1..].iter() {
        parent_indices.push(children.iter().position(|&x| x == *p).unwrap());
    }
    for index in (0..num_nodes).rev() {
        end_indices[index] = std::cmp::max(index + 1, end_indices[index]);
        end_indices[parent_indices[index]] =
            std::cmp::max(end_indices[index], end_indices[parent_indices[index]]);
    }
    end_indices
}

pub(crate) fn pop_jump_push_main(
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    output: u8,
    reps: u32,
) {
    /*! Rust doesn't have stable generators so the whole tree gets processed with 'visits'. */
    let ideals_count = count_subtrees(children[0], parents, children);
    let num_nodes = children.len();
    let ttl_ideals = ideals_count as f64 * reps as f64;
    println!(
        "Generating {ideals_count} ideals from {num_nodes} nodes {reps} times ({ttl_ideals}).\n"
    );

    let args = prep_args(root, parents, children, output);

    let start_time = Instant::now();
    let mut time_delta = f64::MAX;
    let mut i = 0;
    while i < reps {
        i += 1;
        let (num_nodes, mut subtree_indices, jump_indices, labels) = args.clone();
        let run_start_time = Instant::now();
        pop_jump_push(
            num_nodes,
            &mut subtree_indices,
            &jump_indices,
            &labels,
            output,
        );
        let run_time_delta = run_start_time.elapsed().as_secs_f64();
        time_delta = if time_delta < run_time_delta {
            time_delta
        } else {
            run_time_delta
        }
    }
    let end_time_delta = start_time.elapsed().as_secs_f64();

    println!("\tCompleted generating ideals...");
    println!("\tAvg Duration per tree {}", end_time_delta / reps as f64);
    println!("\tBest Duration per tree {time_delta}");
    println!(
        "\t{} ns avg per ideal",
        (end_time_delta / reps as f64) / ideals_count as f64 * 1e9
    );
    println!(
        "\t{} ns best per ideal\n",
        time_delta / ideals_count as f64 * 1e9
    );
}
