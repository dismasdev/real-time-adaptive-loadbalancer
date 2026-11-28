use criterion::{Criterion, criterion_group, criterion_main};

use std::hint::black_box;

fn throughput_benchmark(c: &mut Criterion) {
    c.bench_function("throughput_placeholder", |b| {
        b.iter(|| {
            black_box((0..1000).sum::<u64>());
        });
    });
}

criterion_group!(benches, throughput_benchmark);
criterion_main!(benches);
