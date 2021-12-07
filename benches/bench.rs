use aoc21_rs::{day06, day07};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn cumulative(c: &mut Criterion) {
    c.bench_function("day 06", |b| {
        let input = day06::parse_input(include_str!("../inputs/day06.txt"));

        b.iter(|| {
            day06::part_1(black_box(input.clone()));
            day06::part_2(black_box(input.clone()));
        });
    });

    c.bench_function("day 07", |b| {
        let input = day07::parse_input(include_str!("../inputs/day07.txt"));

        b.iter(|| {
            day07::part_1(black_box(&input));
            day07::part_2(black_box(&input));
        });
    });
}

criterion_group!(benches, cumulative);
criterion_main!(benches);
