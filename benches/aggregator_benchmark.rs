use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heiko_hash_task::{aggregator::{aggregate_hashes, aggregate_hashes_by_parts}, helpers::generate_random_hashes};

fn benchmark_aggregate_hashes(c: &mut Criterion) {
    let hashes = generate_random_hashes();

    c.bench_function("aggregate hashes", |b| {
        b.iter(|| aggregate_hashes(black_box(&hashes)))
    });

    c.bench_function("aggregate_hashes_by_parts", |b| {
        b.iter(|| aggregate_hashes_by_parts(black_box(&hashes)))
    });
}

criterion_group!(benches, benchmark_aggregate_hashes,);
criterion_main!(benches);
