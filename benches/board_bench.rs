#![allow(dead_code, unused_macros)]
use criterion::{criterion_group, criterion_main, Criterion};

use board::{Algorithm, Board};
use tnewt_board::*;

#[rustfmt::skip]
macro_rules! old {
    () => { Board::new() };
    (name) => { "Name1" };
}
#[rustfmt::skip]
macro_rules! new {
    () => { Board::new() };
    (name) => { "Name2" };
}
#[rustfmt::skip]
macro_rules! add_bench_to_group { // {{{
    ($group:expr, $func:expr, $algorithm:expr, $board:expr, $name:expr) => {
        $group.bench_function($name, |b| {
            b.iter(|| {
                $board.set_algorithm(Algorithm::Clone);
                ($func)(&mut $board)
            })
        });
    };
}
macro_rules! _add_bench {
    ($c:expr, $func:expr) => {
        if BENCH_ALGORITHM == BenchAlgorithms::Both || BENCH_ALGORITHM == BenchAlgorithms::Clone {
            let mut clone_group = $c.benchmark_group(stringify!($func).to_string() + " (Clone)");
            add_bench_to_group!(clone_group, $func, Algorithm::Clone, old!(), old!(name));
            add_bench_to_group!(clone_group, $func, Algorithm::Clone, new!(), new!(name));
            clone_group.finish();
        }

        if BENCH_ALGORITHM == BenchAlgorithms::Both || BENCH_ALGORITHM == BenchAlgorithms::Unmove {
            let mut clone_group = $c.benchmark_group(stringify!($func).to_string() + " (Unmove)");
            add_bench_to_group!(clone_group, $func, Algorithm::Clone, old!(), old!(name));
            add_bench_to_group!(clone_group, $func, Algorithm::Clone, new!(), new!(name));
            clone_group.finish();
        }
    };
}

macro_rules! create_bench_function {
    ($name:ident, |$board:ident| $body:expr) => {
        #[allow(dead_code)]
        fn $name($board: &mut Board) {
            $body
        }
    };
} // }}}

#[derive(PartialEq)]
enum BenchAlgorithms {
    Clone,
    Unmove,
    Both,
}
fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! add_benches {// {{{
        ($($f:expr),+) => {
            $( _add_bench!(c, $f); )+
        };
    } // }}}

    const BENCH_ALGORITHM: BenchAlgorithms = BenchAlgorithms::Both;
    create_bench_function!(new_board_moves, |board| {
        board.num_legal_moves().unwrap();
    });

    create_bench_function!(random_game, |board| {
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    });

    create_bench_function!(depth_pos_count, |board| {
        for i in 0..5 {
            board.depth_num_positions(i).unwrap();
        }
    });

    // add_benches![random_game, new_board_moves];
    add_benches![depth_pos_count];
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
