use codspeed_aoc::day17;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D17_INPUT: &str = include_str!("../input/2024/day17.txt");

pub fn d17p1(c: &mut Criterion) {
    c.bench_function("17p1", |b| {
        b.iter(|| black_box(day17::part1(black_box(D17_INPUT))))
    });
}

pub fn d17p2(c: &mut Criterion) {
    c.bench_function("17p2", |b| {
        b.iter(|| black_box(day17::part2(black_box(D17_INPUT))))
    });
}

criterion_group!(d17, d17p1, d17p2);
criterion_main!(d17);
