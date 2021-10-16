use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("full_document.md");
    c.bench_function("full_document", |b| b.iter(|| dgmark::parse(input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
