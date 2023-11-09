use criterion::{criterion_group, criterion_main, Criterion};

use tnewt_board::Board;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("New Board Moves", |b| b.iter(|| {
        let _ = Board::new().generate_moves();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
