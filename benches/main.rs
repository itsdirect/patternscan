use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use patternscan::{Horspool, NaiveSearch, Pattern};

const PATTERN: &str = "01 01 01 01 01 01 01 01";

fn naive_search(b: &mut Bencher, pattern: &Pattern, data: &[u8]) {
    b.iter(move || {
        pattern.matches::<NaiveSearch>(data).next().unwrap();
    });
}

fn horspool(b: &mut Bencher, pattern: &Pattern, data: &[u8]) {
    b.iter(move || {
        pattern.matches::<Horspool>(data).next().unwrap();
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut data = vec![0; 1_000_000];
    let len = data.len();
    data[len - 8..].fill(1);
    let pattern: Pattern = PATTERN.parse().unwrap();

    c.bench_function("naive_search", |b| naive_search(b, &pattern, &data));
    c.bench_function("horspool", |b| horspool(b, &pattern, &data));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
