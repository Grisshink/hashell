use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashell::Hashell;

fn hashell_benchmark(c: &mut Criterion) {
    c.benchmark_group("hashell")
        .sample_size(1000)
        .bench_function("hashell len 16", |b| {
            let hasher = Hashell::new(16);
            let mut i = 0;
            b.iter(|| {
                i += 1;
                hasher.digest(black_box(i).to_string().as_str())
            })
        })
        .bench_function("hashell len 256", |b| {
            let hasher = Hashell::new(256);
            let mut i = 0;
            b.iter(|| {
                i += 1;
                hasher.digest(black_box(i).to_string().as_str())
            })
        });
}

criterion_group!(benches, hashell_benchmark);
criterion_main!(benches);
