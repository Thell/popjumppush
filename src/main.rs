use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use clap::Parser;

mod koda_ruskey;
mod node_manipulation;
mod pop_jump_push;
mod pop_jump_push_par;
mod sample_data;

use koda_ruskey::koda_ruskey_main;
use node_manipulation::arrange_largest_subtrees;
use pop_jump_push::pop_jump_push_main;
use pop_jump_push::pop_jump_push_unsafe_main;
use pop_jump_push_par::pop_jump_push_par_main;
use pop_jump_push_par::pop_jump_push_unsafe_par_main;
use sample_data::get_sample_data;

pub(crate) type BoxedErr = Box<dyn std::error::Error>;

/// This is a small app for exploring and comparing an implementation of
/// the Pop Jump Push algorithm and the Koda Ruskey Ideals from Forest
/// Posets Algorithm (As described in Knuth's TAOCP Volume 4A 7.2.11).
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// algorithm to use.
    ///    0 = Pop Jump Push
    ///    1 = Koda Ruskey
    ///    2 = both
    #[arg(short, long, verbatim_doc_comment)]
    algo: u8,

    #[arg(short, long, help = "See the sample_data.rs file for names.")]
    sample_set: String,

    /// 0 = performance timing
    /// 1 = algorithm arguments dump
    /// 2 = raw algorithm output
    /// 3 = ideals node output in native algorithm order
    /// 4 = ideals node output from native to sorted preorder (for comparing output)
    #[arg(short, long, verbatim_doc_comment)]
    output: u8,

    /// The number of times to perform the performance test.
    /// (0 produces summary)
    #[arg(short, long, verbatim_doc_comment)]
    reps: u32,

    /// The max number of workers to use. This only applies to Pop Jump Push.
    /// ( 0 or 1 is the same as omitting)
    #[arg(short, long, verbatim_doc_comment)]
    workers: Option<u8>,

    /// The arrange tree by size of subtrees.
    /// (true = largest rightmost, false = largest leftmost)
    #[arg(long, verbatim_doc_comment)]
    arrange_right: Option<bool>,
}

fn dump_args(
    algos: Vec<&str>,
    root: usize,
    parents: &Vec<usize>,
    children: &Vec<usize>,
    max_workers: u8,
) {
    println!("\n=== Test Set Data ===");
    let arg = "root";
    println!("{arg:>18}: {root}");
    let arg = "parents";
    println!("{arg:>18}: {parents:?}");
    let arg = "children";
    println!("{arg:>18}: {children:?}\n");

    for algo in algos.into_iter() {
        println!("=== {algo} ===");
        if algo == "pop_jump_push" {
            if max_workers < 2 {
                let _ = pop_jump_push::prep_args(root, parents, children, 1);
            } else {
                let _ = pop_jump_push_par::prep_args(root, parents, children, 1, max_workers);
            }
        } else {
            let _ = koda_ruskey::prep_args(root, parents, children, 1);
        }
        println!()
    }
}

fn benchmark(
    algos: Vec<&str>,
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    reps: u32,
    max_workers: u8,
) {
    for algo in algos.into_iter() {
        println!("=== {algo} ===");
        if algo == "pop_jump_push" {
            if max_workers < 2 {
                println!("== safe ==");
                pop_jump_push_main(root, parents, children, 0, reps);
                println!("== unsafe ==");
                pop_jump_push_unsafe_main(root, parents, children, 0, reps);
            } else {
                println!("== safe ==");
                pop_jump_push_par_main(root, parents, children, 0, reps, max_workers);
                println!("== unsafe ==");
                pop_jump_push_unsafe_par_main(root, parents, children, 0, reps, max_workers);
            }
        } else {
            koda_ruskey_main(root, parents, children, 0, reps);
        }
        println!()
    }
}

fn generate_ideals(
    algos: Vec<&str>,
    root: usize,
    parents: &[usize],
    children: &Vec<usize>,
    output: u8,
    max_workers: u8,
) {
    for algo in algos.into_iter() {
        println!("=== {algo} ===");
        if algo == "pop_jump_push" {
            if max_workers < 2 {
                pop_jump_push_main(root, parents, children, output, 1);
            } else {
                pop_jump_push_par_main(root, parents, children, output, 1, max_workers);
            }
        } else {
            koda_ruskey_main(root, parents, children, output, 1);
        }
        println!()
    }
}
fn main() -> Result<(), BoxedErr> {
    let args = Cli::parse();

    let algos = match args.algo {
        0 => vec!["pop_jump_push"],
        1 => vec!["koda_ruskey"],
        _ => vec!["pop_jump_push", "koda_ruskey"],
    };
    let (root, mut parents, mut children) = get_sample_data(&args.sample_set);
    if let Some(arrangement) = args.arrange_right {
        (parents, children) = arrange_largest_subtrees(root, &parents, &children, arrangement);
    }

    let output = args.output;
    let reps = args.reps;
    let max_workers = match args.workers {
        Some(x) => x,
        _ => 1,
    };

    match output {
        0 => benchmark(algos, root, &parents, &children, reps, max_workers),
        1 => dump_args(algos, root, &parents, &children, max_workers),
        _ => generate_ideals(algos, root, &parents, &children, output, max_workers),
    }

    Ok(())
}
