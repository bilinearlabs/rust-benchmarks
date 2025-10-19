# rust-benchmarks

This repository presents different Rust benchmarks in an A/B format, showing which one is faster. Each benchmark reports execution time and includes an explanation of the underlying reasons on which approach is better.

The examples follow these principles:
* Presents two approaches (A/B) that solve the same problem.
* The examples are small and targeting a very specific functionality.
* Being faster doesn't always mean it's the right choice, each use case has its needs.
* Code is kept minimal, no extra boilerplate, just the benchmark.
* One file, one benchmark. Each file contains two functions to be benchmaked.
* Avoid files longer than 100 lines.
* The code of each file is self contained, no need to set up a project. See `cargo install rust-script`.
* Benchmarks aim to address real problems when possible, but some are for purely educational purposes.
* Each code file should include documentation explaining why one benchmark is slower or faster than the other.
* Benchmarks are run with `criterion`. Use `black_box` to prevent the compiler from optimizing away computations.
* Both Rust core language features and external crates can be benchmarked.
* Preferably shall not require a lot of domian specific knowledge of the problem.
* Some benchmarks may depend on the compiler and optimization level being used.

# run

First, install the [rust-script](https://rust-script.org/) crate. It allows you to embed cargo manifests within the same `.rs` file, enabling a single self-contained code with everything you need.

```
cargo install rust-script
```

And now you can run any of the benchmarks as follows. Replace the name by any file in the `benchmarks` folder.

```
rust-script ./benchmarks/branching_vs_xor.rs
```

* TODO: Measure memory footprint.
* TODO: Measure CPU cycles.