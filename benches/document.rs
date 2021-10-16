use criterion::{criterion_group, criterion_main, Criterion, Throughput};

pub fn text_with_productlists(c: &mut Criterion) {
    let input = include_str!("inputs/text_with_productlists.md");
    let mut group = c.benchmark_group("document");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("text with product lists", |b| {
        b.iter(|| dgmark::parse(input))
    });

    group.finish();
}

criterion_group!(benches, text_with_productlists);
criterion_main!(benches);
