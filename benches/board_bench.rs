use criterion::{criterion_group, criterion_main, Criterion};

use tnewt_board::*;
use board::{Playable, Algorithm};

fn board_a() -> impl Playable { board::new::<implementations::mut_pass::Board>() }
fn a_name() -> String { "MutPass".into() }

fn board_b() -> impl Playable { board::new::<implementations::retain::Board>() }
fn b_name() -> String { "Retain".into() }

fn criterion_benchmark(c: &mut Criterion) {
    let mut new_board_moves = c.benchmark_group("New Board Moves");
    new_board_moves.bench_function(a_name(), |b| b.iter(|| {
        let _ = board_a().dbg_move_count();
    }));
    new_board_moves.bench_function(b_name(), |b| b.iter(|| {
        let _ = board_b().dbg_move_count();
    }));
    new_board_moves.finish();


    // TODO: Make into macro
    let mut clone_random_games = c.benchmark_group("Play Random Game (Clone)");
    clone_random_games.bench_function(a_name(), |b| b.iter(|| {
        let mut board = board_a();
        board.dbg_set_algorithm(Algorithm::Clone);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    clone_random_games.bench_function(b_name(), |b| b.iter(|| {
        let mut board = board_b();
        board.dbg_set_algorithm(Algorithm::Clone);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    clone_random_games.finish();

    let mut unmove_random_games = c.benchmark_group("Play Random Game (Unmove)");
    unmove_random_games.bench_function(a_name(), |b| b.iter(|| {
        let mut board = board_a();
        board.dbg_set_algorithm(Algorithm::Unmove);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    unmove_random_games.bench_function(b_name(), |b| b.iter(|| {
        let mut board = board_b();
        board.dbg_set_algorithm(Algorithm::Unmove);
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    unmove_random_games.finish();


    let mut clone_depth_pos_count = c.benchmark_group("Depth Position Count (Clone)");
    clone_depth_pos_count.bench_function(a_name(), |b| b.iter(|| {
        let mut board = board_a();
        board.dbg_set_algorithm(Algorithm::Unmove);
        for i in 0..5 {
            board.dbg_depth_num_positions(i).unwrap();
        };
    }));
    clone_depth_pos_count.bench_function(b_name(), |b| b.iter(|| {
        let mut board = board_b();
        board.dbg_set_algorithm(Algorithm::Unmove);
        for i in 0..5 {
            board.dbg_depth_num_positions(i).unwrap();
        };
    }));
    clone_depth_pos_count.measurement_time(core::time::Duration::from_secs(20));
    clone_depth_pos_count.finish();

    let mut unmove_depth_pos_count = c.benchmark_group("Depth Position Count (Unmove)");
    unmove_depth_pos_count.bench_function(a_name(), |b| b.iter(|| {
        let mut board = board_a();
        board.dbg_set_algorithm(Algorithm::Clone);
        for i in 0..5 {
            board.dbg_depth_num_positions(i).unwrap();
        };
    }));
    unmove_depth_pos_count.bench_function(b_name(), |b| b.iter(|| {
        let mut board = board_b();
        board.dbg_set_algorithm(Algorithm::Clone);
        for i in 0..5 {
            board.dbg_depth_num_positions(i).unwrap();
        };
    }));
    unmove_depth_pos_count.measurement_time(core::time::Duration::from_secs(20));
    unmove_depth_pos_count.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
