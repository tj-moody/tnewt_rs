use criterion::{criterion_group, criterion_main, Criterion};

use tnewt_board::*;
use board::{Playable, Algorithm};

fn old_board() -> impl Playable { board::new::<implementations::retain::Board>() }
fn old_name() -> String { "MutPass".into() }

fn new_board() -> impl Playable { board::new::<implementations::retain::Board>() }
fn new_name() -> String { "Retain".into() }

fn criterion_benchmark(c: &mut Criterion) {
    let mut new_board_moves = c.benchmark_group("New Board Moves");
    new_board_moves.bench_function(old_name(), |b| b.iter(|| {
        let _ = old_board().num_legal_moves();
    }));
    new_board_moves.bench_function(new_name(), |b| b.iter(|| {
        let _ = new_board().num_legal_moves();
    }));
    new_board_moves.finish();


    // TODO: Make into macro
    let mut clone_random_games = c.benchmark_group("Play Random Game (Clone)");
    clone_random_games.bench_function(old_name(), |b| b.iter(|| {
        let mut board = old_board();
        board.set_algorithm(Algorithm::Clone);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    clone_random_games.bench_function(new_name(), |b| b.iter(|| {
        let mut board = new_board();
        board.set_algorithm(Algorithm::Clone);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    clone_random_games.finish();

    let mut unmove_random_games = c.benchmark_group("Play Random Game (Unmove)");
    unmove_random_games.bench_function(old_name(), |b| b.iter(|| {
        let mut board = old_board();
        board.set_algorithm(Algorithm::Unmove);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    unmove_random_games.bench_function(new_name(), |b| b.iter(|| {
        let mut board = new_board();
        board.set_algorithm(Algorithm::Unmove);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    unmove_random_games.finish();


    let mut clone_depth_pos_count = c.benchmark_group("Depth Position Count (Clone)");
    clone_depth_pos_count.bench_function(old_name(), |b| b.iter(|| {
        let mut board = old_board();
        board.set_algorithm(Algorithm::Unmove);
        for i in 0..5 {
            board.depth_num_positions(i).unwrap();
        };
    }));
    clone_depth_pos_count.bench_function(new_name(), |b| b.iter(|| {
        let mut board = new_board();
        board.set_algorithm(Algorithm::Unmove);
        for i in 0..5 {
            board.depth_num_positions(i).unwrap();
        };
    }));
    clone_depth_pos_count.measurement_time(core::time::Duration::from_secs(20));
    clone_depth_pos_count.finish();

    let mut unmove_depth_pos_count = c.benchmark_group("Depth Position Count (Unmove)");
    unmove_depth_pos_count.bench_function(old_name(), |b| b.iter(|| {
        let mut board = old_board();
        board.set_algorithm(Algorithm::Clone);
        for i in 0..5 {
            board.depth_num_positions(i).unwrap();
        };
    }));
    unmove_depth_pos_count.bench_function(new_name(), |b| b.iter(|| {
        let mut board = new_board();
        board.set_algorithm(Algorithm::Clone);
        for i in 0..5 {
            board.depth_num_positions(i).unwrap();
        };
    }));
    unmove_depth_pos_count.measurement_time(core::time::Duration::from_secs(20));
    unmove_depth_pos_count.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
