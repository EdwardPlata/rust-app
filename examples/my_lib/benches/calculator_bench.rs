use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_lib::{factorial, fibonacci, Calculator};

fn bench_calculator_operations(c: &mut Criterion) {
    let calc = Calculator::new("Benchmark Calculator");

    c.bench_function("add", |b| {
        b.iter(|| calc.add(black_box(42.0), black_box(24.0)))
    });

    c.bench_function("multiply", |b| {
        b.iter(|| calc.multiply(black_box(42.0), black_box(24.0)))
    });

    c.bench_function("divide", |b| {
        b.iter(|| calc.divide(black_box(42.0), black_box(24.0)).unwrap())
    });
}

fn bench_factorial(c: &mut Criterion) {
    c.bench_function("factorial_10", |b| b.iter(|| factorial(black_box(10))));

    c.bench_function("factorial_20", |b| b.iter(|| factorial(black_box(20))));
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci_10", |b| b.iter(|| fibonacci(black_box(10))));

    c.bench_function("fibonacci_20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(
    benches,
    bench_calculator_operations,
    bench_factorial,
    bench_fibonacci
);
criterion_main!(benches);
