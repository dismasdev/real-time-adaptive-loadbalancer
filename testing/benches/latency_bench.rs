use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

use std::hint::black_box;

fn latency_benchmark(c: &mut Criterion) {
    c.bench_function("latency_placeholder", |b| {
        b.iter(|| {
            black_box(100_u64);
        });
    });
}

criterion_group!(benches, latency_benchmark);
criterion_main!(benches);