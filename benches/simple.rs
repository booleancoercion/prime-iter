use prime_iter::primes;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("primes up to 100_000", |b| {
        b.iter(|| {
            primes::<i32>().take_while(|&x| x < 100_000).for_each(|x| {
                black_box(x);
            })
        })
    });
    c.bench_function("primes up to 1_000_000", |b| {
        b.iter(|| {
            primes::<i32>()
                .take_while(|&x| x < 1_000_000)
                .for_each(|x| {
                    black_box(x);
                })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
