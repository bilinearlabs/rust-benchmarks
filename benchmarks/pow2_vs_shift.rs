//! ```cargo
//! [dependencies]
//! criterion = { version = "0.5", default-features = false}
//! ```

// Note: This is a Rust self contained program. If you have rust-script installed
// you can run it with:
// rust-script pow2_vs_shift.rs
use criterion::{black_box, Criterion};

/// ❌ Slow. Computes 2^exp using integer pow. This is a slow operation.
fn a_pow2_pow(exp: u32) -> u64 {
    2u64.pow(exp)
}

/// ✅ Fast. Computes 2^exp by shifting 1 left by exp bits.
/// Example:
/// 1 << 3 = 1000 (1 -> 8)
fn b_pow2_shift(exp: u32) -> u64 {
    1u64 << exp
}

fn main() {
    let mut c = Criterion::default()
        .sample_size(20)
        .warm_up_time(std::time::Duration::from_millis(500));

    // Generate exponents in a safe range for u64 (0..=63)
    let inputs: Vec<u32> = (0..=63).collect();

    c.bench_function("pow2 via pow (2u64.pow(exp))", |b| {
        b.iter(|| {
            for exp in &inputs {
                let _ = a_pow2_pow(black_box(*exp));
            }
        });
    });

    c.bench_function("pow2 via shift (1u64 << exp)", |b| {
        b.iter(|| {
            for exp in &inputs {
                let _ = b_pow2_shift(black_box(*exp));
            }
        });
    });

    c.final_summary();
}


