use clap::Parser;

mod koda_ruskey;
mod node_manipulation;
mod pop_jump_push;
mod sample_data;

use koda_ruskey::koda_ruskey_main;
use pop_jump_push::pop_jump_push_main;
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
}

fn dump_args(algos: Vec<&str>, root: usize, parents: &Vec<usize>, children: &Vec<usize>) {
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
            let _ = pop_jump_push::prep_args(root, parents, children, 1);
        } else {
            let _ = koda_ruskey::prep_args(root, parents, children, 1);
        }
        println!()
    }
}

fn benchmark(algos: Vec<&str>, root: usize, parents: &[usize], children: &Vec<usize>, reps: u32) {
    for algo in algos.into_iter() {
        println!("=== {algo} ===");
        if algo == "pop_jump_push" {
            pop_jump_push_main(root, parents, children, 0, reps);
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
) {
    for algo in algos.into_iter() {
        println!("=== {algo} ===");
        if algo == "pop_jump_push" {
            pop_jump_push_main(root, parents, children, output, 1);
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
    let (root, parents, children) = get_sample_data(&args.sample_set);
    let output = args.output;
    let reps = args.reps;

    match output {
        0 => benchmark(algos, root, &parents, &children, reps),
        1 => dump_args(algos, root, &parents, &children),
        _ => generate_ideals(algos, root, &parents, &children, output),
    }

    Ok(())
}
