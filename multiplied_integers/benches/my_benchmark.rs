use multiplied_integers::{brute_method, run_all};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash_method", |b| b.iter(|| run_all()));
}
fn criterion_benchmark_brute(c: &mut Criterion) {
    c.bench_function("brute_method", |b| b.iter(|| brute_method::run_brute()));
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_brute);
criterion_main!(benches);
