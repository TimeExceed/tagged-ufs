use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tagged_ufs::UnionFindSets;

criterion_group!(benches, add_union_case);
criterion_main!(benches);

fn add_union_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_union");
    let scales = [1_000, 10_000, 100_000, 200_000, 400_000];
    for n in scales {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, n| {
            b.iter(|| {
                add_union(*n);
            })
        });
    }
    group.finish();
}

fn add_union(n: usize) {
    let mut sets = UnionFindSets::<usize, ()>::new();
    for i in 0..n {
        sets.make_set(i, ()).unwrap();
    }
    for i in 1..n {
        sets.unite(&0, &i).unwrap();
    }
}
