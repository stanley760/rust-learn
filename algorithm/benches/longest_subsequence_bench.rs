use std::hint::black_box;

use criterion::{ criterion_group, criterion_main, Criterion};
use algorithm::bitopts::xor::longest_subsequence::Solution;

fn bench_longest_subsequence(c: &mut Criterion) {
    let mut group = c.benchmark_group("longest_subsequence");
    group.bench_function("v1", |b| {
        b.iter(|| black_box(Solution::longest_subsequence_v1(vec![1, 2, 3])));
    });
    group.bench_function("original", |b| {
        b.iter(|| black_box(Solution::longest_subsequence(vec![1, 2, 3])));
    });
    group.finish();
}

criterion_group!(benches, bench_longest_subsequence);
criterion_main!(benches);
