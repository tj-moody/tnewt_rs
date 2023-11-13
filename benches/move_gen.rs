use criterion::{criterion_group, criterion_main, Criterion};

use tnewt_board::*;

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("New Board Moves", |b| b.iter(|| {
        let _ = Board::new().gen_legal_moves();
    }));
    let mut random_games = c.benchmark_group("Play Random Game");
    random_games.bench_function("Unmove", |b| b.iter(|| {
        let mut board = Board::new();
        board.implementation = Implementation::Unmove;
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    random_games.bench_function("Clone", |b| b.iter(|| {
        let mut board = Board::new();
        board.implementation = Implementation::Clone;
        const MOVE_LIMIT: u32 = 1000;
        board.play_random_game(MOVE_LIMIT).unwrap();
    }));
    random_games.finish();

    let mut depth_pos_count = c.benchmark_group("Depth Position Count");
    depth_pos_count.bench_function("Unmove", |b| b.iter(|| {
        let mut board = Board::new();
        board.implementation = Implementation::Unmove;
        for i in 0..5 {
            board.dbg_depth_num_positions(i).unwrap();
        };
    }));
    depth_pos_count.bench_function("Clone", |b| b.iter(|| {
        let mut board = Board::new();
        board.implementation = Implementation::Clone;
        for i in 0..5 {
            board.dbg_depth_num_positions(i).unwrap();
        };
    }));
    depth_pos_count.measurement_time(core::time::Duration::from_secs(20));
    depth_pos_count.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
