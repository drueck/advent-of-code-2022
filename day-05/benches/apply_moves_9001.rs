use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_05::{apply_moves_9001, parse_input};
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let (original_stacks, moves) = parse_input(&input);
    c.bench_function("apply_moves_9001", |b| {
        b.iter_batched(
            || original_stacks.clone(),
            |mut stacks| {
                apply_moves_9001(black_box(&mut stacks), black_box(&moves));
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
