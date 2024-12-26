use codspeed_aoc::day25;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D25_INPUT: &str = include_str!("../input/2024/day25.txt");

pub fn d25p1(c: &mut Criterion) {
    c.bench_function("25p1", |b| {
        b.iter(|| black_box(day25::part1(black_box(D25_INPUT))))
    });
}

criterion_group!(d25, d25p1);
criterion_main!(d25);
