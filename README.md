A chess engine/library for me to optimize, written in Rust.
- Primarily consists of the `tnewt_board` library, which exposes the `Board` struct to handle all game logic.

# Usage
- Run the `cargo test  --package tnewt_board` to run the test suite for board logic correctness.
  - Compares the number of legal positions at certain depths for well-known edge cases against known values.
- Run `cargo bench` to run benchmarks for position and legal move generation.
- See documentation for `Board` struct in [tnewt_board/docs.md](https://github.com/tj-moody/tnewt_rs/blob/main/tnewt_board/docs.md)

# Future optimizations
- Increase memory reuse/reduce allocations when traversing position tree
- Optimize memory footprint of board state
- Implement bitboards & magic bitboards

# Future project direction
- Implement an evaluation function and alpha-beta pruning for a fully-fledged chess engine
