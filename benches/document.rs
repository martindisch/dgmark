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

pub fn text_with_quotes(c: &mut Criterion) {
    let input = include_str!("inputs/text_with_quotes.md");
    let mut group = c.benchmark_group("document");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("text with quotes", |b| {
        b.iter(|| dgmark::parse(input))
    });

    group.finish();
}

pub fn text_with_everything(c: &mut Criterion) {
    let input = include_str!("inputs/text_with_everything.md");
    let mut group = c.benchmark_group("document");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("text with text_with_everything", |b| {
        b.iter(|| dgmark::parse(input))
    });

    group.finish();
}

criterion_group!(
    benches,
    text_with_productlists,
    text_with_quotes,
    text_with_everything
);
criterion_main!(benches);
