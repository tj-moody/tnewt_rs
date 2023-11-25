#![allow(dead_code, unused_macros)]
use criterion::{criterion_group, criterion_main, Criterion};

use board::{Algorithm, Playable};
use tnewt_board::{mov::Move, *};

macro_rules! old_implementation {
    () => { new!(threat_squares) };
    (type) => { Vec<Move> };
}
macro_rules! new_implementation {
    () => { new!(more_magic) };
    (type) => { Vec<Move> };
}
fn old_name() -> String {
    "Threat Squares".into()
}

fn new_name() -> String {
    "More Magic".into()
}

fn criterion_benchmark(c: &mut Criterion) {
    #[derive(PartialEq)]
    enum BenchAlgorithms {
        Clone,
        Unmove,
        Both,
    }
    const BENCH_ALGORITHM: BenchAlgorithms = BenchAlgorithms::Both;

    #[rustfmt::skip]
    macro_rules! add_bench_to_group {// {{{
        ($group:expr, $func:expr, $algorithm:expr, $board:expr, $name:expr) => {
            $group.bench_function($name, |b| {
                b.iter(|| {
                    $board.set_algorithm(Algorithm::Clone);
                    ($func)(&mut $board)
                })
            });
        };
    } // }}}
    #[rustfmt::skip]
    macro_rules! add_benches { // {{{
        (old, $( $func:expr),* ) => {
            $(
                let mut clone_group = c.benchmark_group(
                    stringify!($func).to_string()
                );
                if BENCH_ALGORITHM == BenchAlgorithms::Both
                || BENCH_ALGORITHM == BenchAlgorithms::Clone {
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Clone,
                        old_implementation!(), old_name() + " (Clone)"
                    );
                }
                if BENCH_ALGORITHM == BenchAlgorithms::Both
                || BENCH_ALGORITHM == BenchAlgorithms::Unmove {
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Unmove,
                        old_implementation!(), old_name() + " (Unmove)"
                    );
                }
                clone_group.finish();
            )*
        };
        ($( $func:expr),* ) => {
            $(
                if BENCH_ALGORITHM == BenchAlgorithms::Both
                || BENCH_ALGORITHM == BenchAlgorithms::Clone {
                    let mut clone_group = c.benchmark_group(
                        stringify!($func).to_string() + " (Clone)"
                    );
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Clone,
                        old_implementation!(), old_name()
                    );
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Clone,
                        new_implementation!(), new_name()
                    );
                    clone_group.finish();
                }

                if BENCH_ALGORITHM == BenchAlgorithms::Both
                || BENCH_ALGORITHM == BenchAlgorithms::Unmove {
                    let mut clone_group = c.benchmark_group(
                        stringify!($func).to_string() + " (Unmove)"
                    );
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Clone,
                        old_implementation!(), old_name()
                    );
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Clone,
                        new_implementation!(), new_name()
                    );
                    clone_group.finish();
                }
            )*
        };
    } // }}}

    macro_rules! create_bench_function {
        ($name:ident, |$board:ident| $body:expr) => {
            #[allow(dead_code)]
            fn $name<T: Clone + IntoIterator<Item = Move>>($board: &mut impl Playable<T>) {
                $body
            }
        };
    }

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

    // add_benches!(random_game, new_board_moves);
    add_benches!(depth_pos_count);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
