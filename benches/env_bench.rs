use criterion::{Criterion, criterion_group, criterion_main};

fn bench_small_env(c: &mut Criterion) {
    c.bench_function("small_env_lookup", |b| {
        b.iter(|| zero_copy_env::get("PATH"))
    });
}

fn bench_std(c: &mut Criterion) {
    c.bench_function("std_env_lookup", |b| b.iter(|| std::env::var("PATH").ok()));
}

criterion_group!(benches, bench_small_env, bench_std);
criterion_main!(benches);
