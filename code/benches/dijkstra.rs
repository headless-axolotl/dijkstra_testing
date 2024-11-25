use code::{dijkstra, graph::*};
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    BatchSize,
    BenchmarkId,
    Criterion
};

pub fn with_density(c: &mut Criterion, density: f32) {
    let mut group = c.benchmark_group(format!("Dijkstra {}% Density", density * 100.0));

    group.sample_size(100);
    group.warm_up_time(std::time::Duration::new(20, 0));
    group.measurement_time(std::time::Duration::new(30, 0));

    const UPPER: usize = 10_000;
    let step = UPPER / 20;

    for i in (step..=UPPER).step_by(step) {
        group.bench_with_input(BenchmarkId::new("Baseline", i), &i, |b, _i| {
            b.iter_batched_ref(
                || Graph::generate_connected(i, density),
                |g| black_box(dijkstra::dijkstra_standard(black_box(g))),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Fibonacci", i), &i, |b, _i| {
            b.iter_batched_ref(
                || Graph::generate_connected(i, density),
                |g| black_box(dijkstra::dijkstra_fibonacci(black_box(g))),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Binary", i), &i, |b, _i| {
            b.iter_batched_ref(
                || Graph::generate_connected(i, density),
                |g| black_box(dijkstra::dijkstra_binary(black_box(g))),
                BatchSize::SmallInput,
            )
        });
    }
}

pub fn density_100(c: &mut Criterion) {
    with_density(c, black_box(1.0));
}

pub fn density_50(c: &mut Criterion) {
    with_density(c, black_box(0.5));
}

pub fn density_20(c: &mut Criterion) {
    with_density(c, black_box(0.2));
}

pub fn density_10(c: &mut Criterion) {
    with_density(c, black_box(0.1));
}

pub fn density_5(c: &mut Criterion) {
    with_density(c, black_box(0.05));
}

pub fn density_1(c: &mut Criterion) {
    with_density(c, black_box(0.01));
}

pub fn density_01(c: &mut Criterion) {
    with_density(c, black_box(0.001));
}

criterion_group!(dijkstra_100, density_100);
criterion_group!(dijkstra_50, density_50);
criterion_group!(dijkstra_20, density_20);
criterion_group!(dijkstra_10, density_10);
criterion_group!(dijkstra_5, density_5);
criterion_group!(dijkstra_1, density_1);
criterion_group!(dijkstra_01, density_01);

criterion_main!(dijkstra_100);

