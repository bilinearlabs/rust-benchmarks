//! ```cargo
//! [dependencies]
//! criterion = { version = "0.5", default-features = false}
//! ```

// Note: This is a Rust self contained program. If you have rust-script installed
// you can run it with:
// rust-script branching_vs_xor.rs

use criterion::{black_box, Criterion};

/// ❌ Slow. This function -1 the input if it is odd, otherwise it adds 1. This is
/// common in merkle tree implementations, where we need to get the sibling of a node.
/// In this case we have an if statement, which is a branch. This branch can be a bit slower
/// in some cases, but note that the compiler can optimize this, so the difference is not always noticeable.
fn a_sibling_if(x: u64) -> u64 {
    if (x & 1) == 1 {
        x - 1
    } else {
        x + 1
    }
}

/// ✅ Fast. It just flips the lowest bit with the XOR operator.
/// The same behaviour but without the branch.
/// Example:
/// 1010 ^ 0001 = 1011 (10 -> 11)
/// 1011 ^ 0001 = 1010 (11 -> 10)
fn b_sibling_xor(x: u64) -> u64 {
    x ^ 1
}

fn main() {
    let mut c = Criterion::default()
        .sample_size(20)
        .warm_up_time(std::time::Duration::from_millis(500));

    // Generate some inputs
    let inputs: Vec<u64> = (1..1_000_000).collect();

    c.bench_function("sibling_if (branching)", |b| {
        b.iter(|| {
            for x in &inputs {
                let _ = a_sibling_if(black_box(*x));
            }
        });
    });

    c.bench_function("sibling_xor (branchless XOR)", |b| {
        b.iter(|| {
            for x in &inputs {
                let _ = b_sibling_xor(black_box(*x));
            }
        });
    });

    c.final_summary();
}
