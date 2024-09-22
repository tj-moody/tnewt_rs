A chess engine/library for me to optimize, written in Rust.
- Primarily consists of the `tnewt_board` library, which exposes the `Board` struct to handle all game logic.

# Usage
- Run the `cargo test  --package tnewt_board` to run the test suite for board logic correctness.
  - Compares the number of legal positions at certain depths for well-known edge cases against known values.
- Run `cargo bench` to run benchmarks for position and legal move generation.
