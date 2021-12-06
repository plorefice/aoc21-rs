use aoc21_rs::day06;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day06(c: &mut Criterion) {
    c.bench_function("day 06", |b| {
        let input = day06::parse_input(include_str!("../inputs/day06.txt"));

        b.iter(|| {
            day06::part_1(black_box(input.clone()));
            day06::part_2(black_box(input.clone()));
        });
    });
}

criterion_group!(benches, day06);
criterion_main!(benches);
