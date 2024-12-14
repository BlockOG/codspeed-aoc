use codspeed_aoc::day13;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D13_INPUT: &str = include_str!("../input/2024/day13.txt");

pub fn d13p1(c: &mut Criterion) {
    c.bench_function("13p1", |b| b.iter(|| black_box(day13::part1(D13_INPUT))));
}

pub fn d13p2(c: &mut Criterion) {
    c.bench_function("13p2", |b| b.iter(|| black_box(day13::part2(D13_INPUT))));
}

criterion_group!(d13, d13p1, d13p2);
criterion_main!(d13);
