//! # Koda-Ruskey Ideals of Forest Posets
//!
//! This module provides an implementation of the Koda-Ruskey Ideals of Forest Posets algorithm,
//! also known as Knuth Algorithm K TAoCP 4A 7.2.1.1.

use std::time::Instant;

use crate::node_manipulation::arrange_by_traversal_post_order;
use crate::node_manipulation::count_subtrees;
use crate::node_manipulation::group_indices_by_value;

fn koda_ruskey(
    active_nodes: &mut [u8],
    focus_pointers: &mut [usize],
    left_child: &[usize],
    fringe_l: &mut [usize],
    fringe_r: &mut [usize],
    labels: &[usize],
    output: u8,
) {
    /*!  - Implements the Koda-Ruskey algorithm.
     */
    loop {
        let mut q = fringe_l[0];
        let p = focus_pointers[q];
        focus_pointers[q] = q;

        if p == 0 {
            return;
        }

        if active_nodes[p] == 0 {
            active_nodes[p] = 1;
            if left_child[p] != 0 {
                q = fringe_r[p];
                fringe_l[q] = p - 1;
                fringe_r[p - 1] = q;
                fringe_r[p] = left_child[p];
                fringe_l[left_child[p]] = p;
            }
        } else {
            active_nodes[p] = 0;
            if left_child[p] != 0 {
                q = fringe_r[p - 1];
                fringe_r[p] = q;
                fringe_l[q] = p;
            }
        }

        focus_pointers[p] = focus_pointers[fringe_l[p]];
        focus_pointers[fringe_l[p]] = fringe_l[p];
        visit(active_nodes, labels, output);
    }
}

pub(crate) fn visit(ideal: &[u8], labels: &[usize], output: u8) {
    /*!  -  Process/output ideals.
     */
    // This just ensures the compiler doesn't optimize anything away during `output == 0` benchmarking.
    let ideal = std::hint::black_box(ideal);

    if output == 2 {
        println!("{ideal:?}")
    } else if output >= 3 {
        let active_indices = ideal
            .iter()
            .skip(1)
            .enumerate()
            .filter(|x| *x.1 == 1u8)
            .map(|x| x.0)
            .collect::<Vec<_>>();
        if output == 3 {
            println!("{active_indices:?}");
        } else {
            let mut result: Vec<_> = active_indices.iter().map(|i| labels[*i]).collect();
            result.sort();
            println!("{result:?}");
        }
    };
}

pub(crate) fn prep_args(
    root: usize,
    parents: &[usize],
    children: &[usize],
    output: u8,
) -> (
    Vec<u8>,
    Vec<usize>,
    Vec<usize>,
    Vec<usize>,
    Vec<usize>,
    Vec<usize>,
) {
    let (parents, children) = arrange_by_traversal_post_order(root, parents, children);
    let labels = children.clone();
    let (root, parents, children) = sorted_post_order_indices(root, &parents, &children);

    let left_child = leftmost_children_indices(root, &parents, &children);
    let left_child = [parents.iter().position(|&x| x == root).unwrap() + 1]
        .iter()
        .chain(left_child.iter())
        .cloned()
        .collect::<Vec<_>>();
    let n = left_child.len() - 1;
    let active_nodes = vec![0; n + 1];
    let focus_pointers = (0..=n).collect::<Vec<_>>();
    let (fringe_l, mut fringe_r) = generate_sibling_arrays(&parents, &children);
    let mut fringe_l = [n]
        .iter()
        .chain(fringe_l.iter())
        .cloned()
        .collect::<Vec<_>>();
    fringe_l[left_child[0]] = 0;
    fringe_r.insert(0, left_child[0]);
    fringe_r[n] = 0;
    if output == 1 {
        let arg = "active_nodes";
        println!("{arg:>18}: {active_nodes:?}");
        let arg = "focus_pointers";
        println!("{arg:>18}: {focus_pointers:?}");
        let arg = "left_child";
        println!("{arg:>18}: {left_child:?}");
        let arg = "fringe_l";
        println!("{arg:>18}: {fringe_l:?}");
        let arg = "fringe_r";
        println!("{arg:>18}: {fringe_r:?}");
    }
    (
        active_nodes,
        focus_pointers,
        left_child,
        fringe_l,
        fringe_r,
        labels,
    )
}

fn generate_sibling_arrays(parents: &[usize], children: &[usize]) -> (Vec<usize>, Vec<usize>) {
    /*!  - Create doubly linked list arrays for left <-> right siblings.
     */
    let n = parents.len();
    let mut left_siblings = vec![0; n];
    let mut right_siblings = vec![0; n];
    let child_indices = group_indices_by_value(parents);

    for (_, children_indices) in child_indices {
        for (i, index) in children_indices.iter().enumerate() {
            if i > 0 {
                left_siblings[*index] = children[children_indices[i - 1]];
            }
            if i < children_indices.len() - 1 {
                right_siblings[*index] = children[children_indices[i + 1]];
            }
        }
    }

    (left_siblings, right_siblings)
}

fn sorted_post_order_indices(
    _root: usize,
    parents: &[usize],
    children: &Vec<usize>,
) -> (usize, Vec<usize>, Vec<usize>) {
    /*!  - Returns sorted postorder children and parents.
     */
    let post_parents = parents
        .iter()
        .map(|p| children.iter().position(|&x| x == *p).unwrap_or(0))
        .collect::<Vec<_>>();
    let post_parents = post_parents
        .into_iter()
        .map(|x| match x {
            0 => 0,
            _ => x + 1,
        })
        .collect::<Vec<_>>();
    let post_children = Vec::from_iter(1..=children.len());
    let post_root = post_children.last().unwrap().to_owned();

    (post_root, post_parents, post_children)
}

fn leftmost_children_indices(_root: usize, parents: &[usize], children: &[usize]) -> Vec<usize> {
    /*! Return an array indicating the index of the left child of each parent.
     */
    children
        .iter()
        .map(|&n| {
            if parents.contains(&n) {
                parents.iter().position(|&x| x == n).unwrap() + 1
            } else {
                0
            }
        })
        .collect()
}

pub(crate) fn koda_ruskey_main(
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    output: u8,
    reps: u32,
) {
    /*! Rust doesn't have stable generators as of yet so this serves as the driver and the whole
     * tree gets processed with 'visits'.
     */

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
        let (mut active_nodes, mut focus_pointers, left_child, mut fringe_l, mut fringe_r, labels) =
            args.clone();
        let run_start_time = Instant::now();
        koda_ruskey(
            &mut active_nodes,
            &mut focus_pointers,
            &left_child,
            &mut fringe_l,
            &mut fringe_r,
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
