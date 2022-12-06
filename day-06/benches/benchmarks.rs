use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use day_06::{index_after_n_unique_characters, index_after_n_unique_characters_hashset};
use std::fs;

fn bench_finders(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let mut group = c.benchmark_group("Finders");
    for n in [4usize, 14usize].iter() {
        group.bench_with_input(BenchmarkId::new("sort and dedup", n), n, |b, n| {
            b.iter(|| index_after_n_unique_characters(black_box(input.as_bytes()), *n))
        });
        group.bench_with_input(BenchmarkId::new("hashset", n), n, |b, n| {
            b.iter(|| index_after_n_unique_characters_hashset(black_box(input.as_bytes()), *n))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_finders);
criterion_main!(benches);

// Output from `cargo bench`:
//
// Finders/sort and dedup/4
//                         time:   [21.506 µs 21.679 µs 21.831 µs]
// Found 11 outliers among 100 measurements (11.00%)
//   2 (2.00%) low severe
//   3 (3.00%) low mild
//   6 (6.00%) high mild
// Finders/hashset/4       time:   [180.22 µs 181.24 µs 182.34 µs]
// Found 6 outliers among 100 measurements (6.00%)
//   5 (5.00%) high mild
//   1 (1.00%) high severe
// Finders/sort and dedup/14
//                         time:   [294.78 µs 297.57 µs 300.50 µs]
// Found 4 outliers among 100 measurements (4.00%)
//   3 (3.00%) high mild
//   1 (1.00%) high severe
// Finders/hashset/14      time:   [655.79 µs 659.97 µs 664.07 µs]
// Found 3 outliers among 100 measurements (3.00%)
//   2 (2.00%) low mild
//   1 (1.00%) high mild
