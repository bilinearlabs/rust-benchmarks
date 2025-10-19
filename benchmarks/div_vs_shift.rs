//! ```cargo
//! [dependencies]
//! criterion = { version = "0.5", default-features = false}
//! ```

// Note: This is a Rust self contained program. If you have rust-script installed
// you can run it with:
// rust-script div_vs_shift.rs
use criterion::{black_box, Criterion};

/// ❌ Slow. Divides the input by 2, using a normal division. This is a slow operation.
/// But note that in many cases the compiler will optimize this to a shift operation.
fn a_halve_div(x: u64) -> u64 {
    x / 2
}

/// ✅ Fast. Shifts the input by 1 bit to the right. This is equivalent to dividing by 2
/// in unsigned integer arithmetic. The same behaviour but without the division.
/// Example:
/// 1010 >> 1 = 0101 (10 -> 5)
/// 0101 >> 1 = 0010 (5 -> 2)
fn b_halve_shr(x: u64) -> u64 {
    x >> 1
}

fn main() {
    let mut c = Criterion::default()
        .sample_size(20)
        .warm_up_time(std::time::Duration::from_millis(500));

    // Generate some inputs
    let inputs: Vec<u64> = (1..1_000_000).collect();

    c.bench_function("halve via division (x / 2)", |b| {
        b.iter(|| {
            for x in &inputs {
                let _ = a_halve_div(black_box(*x));
            }
        });
    });

    c.bench_function("halve via shift (x >> 1)", |b| {
        b.iter(|| {
            for x in &inputs {
                let _ = b_halve_shr(black_box(*x));
            }
        });
    });

    c.final_summary();
}
