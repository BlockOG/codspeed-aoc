use codspeed_aoc::day18;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D18_INPUT: &str = include_str!("../input/2024/day18.txt");

pub fn d18p1(c: &mut Criterion) {
    c.bench_function("18p1", |b| {
        b.iter(|| black_box(day18::part1(black_box(D18_INPUT))))
    });
}

pub fn d18p2(c: &mut Criterion) {
    c.bench_function("18p2", |b| {
        b.iter(|| black_box(day18::part2(black_box(D18_INPUT))))
    });
}

criterion_group!(d18, d18p1, d18p2);
criterion_main!(d18);
