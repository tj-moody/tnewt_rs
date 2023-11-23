use criterion::{criterion_group, criterion_main, Criterion};

use board::{Algorithm, Playable};
use tnewt_board::*;

fn old_board() -> impl Playable {
    board::new::<implementations::retain::Board>()
}
fn old_name() -> String {
    "Retain".into()
}

fn new_board() -> impl Playable {
    board::new::<implementations::pre_filter::Board>()
}
fn new_name() -> String {
    "PreFilter".into()
}

fn criterion_benchmark(c: &mut Criterion) {
    #[derive(PartialEq)]
    enum BenchAlgorithms {
        Clone,
        Unmove,
        Both,
    }
    const BENCH_ALGORITHM: BenchAlgorithms = BenchAlgorithms::Clone;

    #[rustfmt::skip]
    macro_rules! add_bench_to_group {// {{{
        ($group:expr, $func:expr, $algorithm:expr, $board:expr, $name:expr) => {
            $group.bench_function($name(), |b| {
                b.iter(|| {
                    let mut board = $board();
                    board.set_algorithm(Algorithm::Clone);
                    ($func)(&mut board)
                })
            });
        };
    }// }}}
    #[rustfmt::skip]
    macro_rules! create_benches { // {{{
        ( $( $func:expr),* ) => {
            $(
                if BENCH_ALGORITHM == BenchAlgorithms::Both
                || BENCH_ALGORITHM == BenchAlgorithms::Clone {
                    let mut clone_group = c.benchmark_group(
                        stringify!($func).to_string() + " (Clone)"
                    );
                    add_bench_to_group!(
                        clone_group, $func, Algorithm::Clone,
                        old_board, old_name
                    );
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Clone,
                        new_board, new_name
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
                        old_board, old_name
                    );
                    add_bench_to_group!(clone_group,
                        $func, Algorithm::Clone,
                        new_board, new_name
                    );
                    clone_group.finish();
                }
            )*
        };
    } // }}}

    fn new_board_moves(board: &mut impl Playable) {
        board.num_legal_moves().unwrap();
    }

    fn random_game(board: &mut impl Playable) {
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }

    fn depth_pos_count(board: &mut impl Playable) {
        for i in 0..5 {
            board.depth_num_positions(i).unwrap();
        }
    }

    create_benches!(random_game, new_board_moves, depth_pos_count);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
