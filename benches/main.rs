use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use patternscan::Pattern;

const PATTERN: &str = "00 00 00 00 00 00 00 00 ? 00 00 00 00 00 00 01";

fn find_first_match(b: &mut Bencher) {
    let mut data = vec![0; 1_000_000];
    *data.last_mut().unwrap() = 1;
    let pattern: Pattern = PATTERN.parse().unwrap();

    b.iter(move || {
        pattern.matches(&data).next().unwrap();
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("find_first_match", find_first_match);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
