use codspeed_aoc::day14;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D14_INPUT: &str = include_str!("../input/2024/day14.txt");

pub fn d14p1(c: &mut Criterion) {
    c.bench_function("14p1", |b| b.iter(|| black_box(day14::part1(D14_INPUT))));
}

pub fn d14p2(c: &mut Criterion) {
    c.bench_function("14p2", |b| b.iter(|| black_box(day14::part2(D14_INPUT))));
}

criterion_group!(d14, d14p1, d14p2);
criterion_main!(d14);
