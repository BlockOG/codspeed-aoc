use codspeed_aoc::day20;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D20_INPUT: &str = include_str!("../input/2024/day20.txt");

pub fn d20p1(c: &mut Criterion) {
    c.bench_function("20p1", |b| {
        b.iter(|| black_box(day20::part1(black_box(D20_INPUT))))
    });
}

pub fn d20p2(c: &mut Criterion) {
    c.bench_function("20p2", |b| {
        b.iter(|| black_box(day20::part2(black_box(D20_INPUT))))
    });
}

criterion_group!(d20, d20p1, d20p2);
criterion_main!(d20);
