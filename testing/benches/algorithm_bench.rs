use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

use std::hint::black_box;

fn algorithm_benchmark(c: &mut Criterion) {
    c.bench_function("algorithm_placeholder", |b| {
        b.iter(|| {
            black_box(100 * 200);
        });
    });
}

criterion_group!(benches, algorithm_benchmark);
criterion_main!(benches);