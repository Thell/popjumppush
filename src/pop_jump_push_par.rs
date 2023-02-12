//! # Parallel implementation of the Pop Jump Push Algorithm for generating all ideals of an arborescence.
//!

use rayon::prelude::*;
use std::time::Instant;

use crate::node_manipulation::arrange_by_traversal_pre_order;
use crate::node_manipulation::count_subtrees;
use crate::pop_jump_push::generate_jump_indices;

pub(crate) struct ParArg {
    num_nodes: usize,
    sequence_indices: Vec<usize>,
    jump_indices: Vec<usize>,
    stop_index: usize,
    stop_value: usize,
    labels: Vec<usize>,
    worker_id: u8,
    output: u8,
}

struct WorkerDetail {
    sequence_indices: Vec<usize>,
    stop_index: usize,
    stop_value: usize,
    worker_id: u8,
}

fn pop_jump_push_par(arg_set: &ParArg) -> usize {
    /*!  - Implements the Pop Jump Push algorithm that works on chunks.
     *  Loop terminates on the last ideal for the worker so it must be visited outside of the loop.
     */
    let num_nodes = arg_set.num_nodes;
    let mut sequence_indices = arg_set.sequence_indices.clone();
    let jump_indices = arg_set.jump_indices.clone();
    let stop_index = arg_set.stop_index;
    let stop_value = arg_set.stop_value;
    let labels = &arg_set.labels;
    let worker_id = arg_set.worker_id;
    let output = arg_set.output;

    let mut visited_count = 0;
    while sequence_indices.len() > stop_index && sequence_indices[stop_index] >= stop_value {
        visited_count += visit(&sequence_indices, labels, worker_id, output);
        let index = jump_indices[sequence_indices.pop().unwrap()];
        if index < num_nodes {
            sequence_indices.extend(index..num_nodes);
        }
    }
    visited_count + visit(&sequence_indices, labels, worker_id, output)
}

#[inline(always)]
pub(crate) fn visit(ideal: &[usize], labels: &[usize], worker_id: u8, output: u8) -> usize {
    /*!  -  Process/output ideals. */
    // This just ensures the compiler doesn't optimize anything away during `output == 0` benchmarking.
    let ideal = std::hint::black_box(ideal);

    if output == 2 {
        println!("{worker_id:<3}: {ideal:?}")
    } else if output >= 3 {
        let mut result = ideal.iter().map(|i| labels[*i]).collect::<Vec<_>>();
        result.sort();
        println!("{worker_id:<3}: {result:?}");
    };
    1
}

fn get_pop_jump_push_ideals(
    num_nodes: usize,
    sequence_indices: &mut Vec<usize>,
    jump_indices: &[usize],
) -> Vec<Vec<usize>> {
    /*!  - Returns generated ideals. */
    let mut ideals = vec![];
    while !sequence_indices.is_empty() {
        ideals.push(sequence_indices.clone());
        let index = jump_indices[sequence_indices.pop().unwrap()];
        if index < num_nodes {
            sequence_indices.extend(index..num_nodes);
        }
    }
    ideals
}

pub(crate) fn prep_args(
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    output: u8,
    max_workers: u8,
) -> Vec<ParArg> {
    /*!  -  Return a tuple of the arguments for pop_jump_push_par. */
    let num_nodes = children.len();
    let (parents, children) = arrange_by_traversal_pre_order(root, parents, children);
    let labels = children.clone();
    let jump_indices = generate_jump_indices(&parents, &children);
    let worker_details = get_worker_details(&jump_indices, max_workers as usize);

    let args: Vec<ParArg> = worker_details
        .iter()
        .map(|wd| ParArg {
            num_nodes,
            sequence_indices: wd.sequence_indices.to_owned(),
            jump_indices: jump_indices.clone(),
            stop_index: wd.stop_index,
            stop_value: wd.stop_value,
            labels: labels.clone(),
            worker_id: wd.worker_id,
            output,
        })
        .collect();

    if output == 1 {
        let arg = "num_nodes";
        println!("{arg:>18}: {num_nodes}");
        let arg = "jump_indices";
        println!("{arg:>18}: {jump_indices:?}");

        println!("\t*** {:?} workers ***", args.len());
        for arg_set in args.iter() {
            let arg = ["Worker ", &arg_set.worker_id.to_string()].concat();
            print!("{arg:>18}: ");
            let arg = arg_set.stop_index;
            print!("stop_index: {arg:>3}");
            let arg = arg_set.stop_value;
            println!(" stop_value: {arg:>3}");
            let arg = "";
            println!("{arg:>18}  indices: {:?} ", &arg_set.sequence_indices);
        }
    }

    args
}

pub(crate) fn pop_jump_push_par_main(
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    output: u8,
    reps: u32,
    max_workers: u8,
) {
    /*! Rust doesn't have stable generators so the whole tree gets processed with 'visits'. */
    let args = prep_args(root, parents, children, output, max_workers);

    let num_nodes = children.len();
    let num_workers = args.len();
    let ideals_count = count_subtrees(children[0], parents, children);
    let ttl_ideals = ideals_count as f64 * reps as f64;
    print!("Generating {ideals_count} ideals from {num_nodes} nodes using ");
    println!("{num_workers} workers {reps} times ({ttl_ideals}).\n");

    let mut performance_data = vec![];
    let mut best_rep_time_delta = f64::MAX;
    let overall_start_time = Instant::now();
    for _ in 0..reps {
        let rep_start_time = Instant::now();
        performance_data = args
            .par_iter()
            .map(|arg_set| {
                let start_time = Instant::now();
                let ideals_count = pop_jump_push_par(arg_set);
                let delta = start_time.elapsed();
                (arg_set.worker_id, delta, ideals_count)
            })
            .collect::<Vec<_>>();
        let rep_time_delta = rep_start_time.elapsed().as_secs_f64();
        if rep_time_delta < best_rep_time_delta {
            best_rep_time_delta = rep_time_delta
        }
    }
    let overall_time_delta = overall_start_time.elapsed().as_secs_f64();

    println!("\tCompleted generating ideals...");
    println!(
        "\tAvg Duration per tree {}",
        overall_time_delta / reps as f64
    );
    println!("\tBest Duration per tree {best_rep_time_delta}");
    println!(
        "\t{} ns avg per ideal",
        (overall_time_delta / reps as f64) / ideals_count as f64 * 1e9
    );
    println!(
        "\t{} ns best per ideal\n",
        best_rep_time_delta / ideals_count as f64 * 1e9
    );

    println!("\tWorkers summary for the last rep...");
    performance_data.sort_by_key(|x| x.0);
    for (id, delta, ideal_count) in performance_data.iter() {
        let delta = delta.as_secs_f64();
        println!(
            "\t\tworker {id:<3} generated {ideal_count} ideals in {delta} for {:?} (ns)",
            delta / *ideal_count as f64 * 1e9,
        )
    }
    let generated_count: usize = performance_data
        .iter()
        .map(|(_, _, ideal_count)| ideal_count)
        .sum();
    println!("\t\tWorkers    generated {generated_count} ideals.\n");
}

fn generate_worker_ideal_prefixes(jump_indices: &[usize], num_workers: usize) -> Vec<Vec<usize>> {
    /*  - Returns the ideal prefixes for each worker
    Consume leading indices in the pre-ordered tree.
    Use the distinct subtrees from them as worker prefixes.
    Limit nodes used to those that generate <= max_num_workers distinct prefixes.
    */
    let num_nodes = jump_indices.len();
    let max_num_workers = *vec![16, num_nodes / 2, num_workers].iter().min().unwrap();

    let mut ideal_prefixes = Vec::new();
    for n in 1..=max_num_workers {
        let mut sequence_indices = (0..n).collect::<Vec<_>>();
        let prefixes = get_pop_jump_push_ideals(n, &mut sequence_indices, &jump_indices[0..n])
            .into_iter()
            .map(|prefix| prefix.to_vec())
            .collect::<Vec<_>>();
        if prefixes.len() > max_num_workers {
            break;
        }
        ideal_prefixes = prefixes;
    }

    ideal_prefixes
}

fn get_worker_details(jump_indices: &[usize], num_workers: usize) -> Vec<WorkerDetail> {
    /*! - Returns worker details controlling which ideals a worker generates. */

    let ideal_prefixes = generate_worker_ideal_prefixes(jump_indices, num_workers);

    // The first prefix contains all of the leading nodes.
    // Generation is done in reverse pre-order.
    // A hard minimum index limit of the node after the leading is
    let hard_suffix_min_index = ideal_prefixes[0].last().unwrap() + 1;
    let base_prefix_indices = (0..hard_suffix_min_index).collect::<Vec<_>>();

    // The disabled nodes in a prefix might disable nodes beyond the min suffix index so we need to
    // identify each prefixes independent min suffix index, the value at the index is set such that
    // subtree generation will terminate when the reverse pre-order processing completes the suffix.
    // A prefix and all indices following its min suffix index are a workers starting subtree.
    let num_nodes = jump_indices.len();
    let tree_indices = (0..num_nodes).collect::<Vec<_>>();
    let mut worker_details = vec![];
    for (worker_id, prefix) in ideal_prefixes.iter().enumerate() {
        let deactivated_nodes = base_prefix_indices
            .iter()
            .filter(|i| !prefix.contains(i))
            .cloned()
            .collect::<Vec<_>>();

        let stop_value = std::cmp::max(
            deactivated_nodes
                .iter()
                .map(|i| jump_indices[*i])
                .max()
                .unwrap_or(hard_suffix_min_index),
            hard_suffix_min_index,
        );

        let stop_index = prefix.len();
        let sequence_indices = prefix
            .iter()
            .chain(tree_indices[stop_value..].iter())
            .cloned()
            .collect();

        worker_details.push(WorkerDetail {
            sequence_indices,
            stop_index,
            stop_value,
            worker_id: worker_id.try_into().unwrap(),
        });
    }

    worker_details
}
