use codspeed_aoc::day19;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D19_INPUT: &str = include_str!("../input/2024/day19.txt");

pub fn d19p1(c: &mut Criterion) {
    c.bench_function("19p1", |b| {
        b.iter(|| black_box(day19::part1(black_box(D19_INPUT))))
    });
}

pub fn d19p2(c: &mut Criterion) {
    c.bench_function("19p2", |b| {
        b.iter(|| black_box(day19::part2(black_box(D19_INPUT))))
    });
}

criterion_group!(d19, d19p1, d19p2);
criterion_main!(d19);
