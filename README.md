# cargo example

A minimal cargo example.

## Requirements

Rust & Cargo.

## Instructions

1. Optional: `cargo --version` to check that cargo is available.
2. `cargo build` to compile. The only source file is at `src/main.rs`. The executable target will be generated at `./target/debug/cargo-example`.
3. Run the executable target with `./target/debug/cargo-example`. The expected out should be a single number `27`.

## THIR ("Typed High-Level Intermediate Representation")

To obtain the THIR of this RUST project, run 
```bash
cargo rustc -- -Zunpretty=thir-tree > thir-dump
```
The THIR will be dumped into the file `thir-dump` from the root.

Note that to obtain the THIR, rustup needs to be updated and defaulted to its nightly version.

To install rustup nightly:
```bash
rustup toolchain install nightly
```

To default to rustup nightly (if you have multiple versions of rustup):
```bash
rustup default nightly
```
