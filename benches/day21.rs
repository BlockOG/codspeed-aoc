use codspeed_aoc::day21;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D21_INPUT: &str = include_str!("../input/2024/day21.txt");

pub fn d21p1(c: &mut Criterion) {
    c.bench_function("21p1", |b| {
        b.iter(|| black_box(day21::part1(black_box(D21_INPUT))))
    });
}

pub fn d21p2(c: &mut Criterion) {
    c.bench_function("21p2", |b| {
        b.iter(|| black_box(day21::part2(black_box(D21_INPUT))))
    });
}

criterion_group!(d21, d21p1, d21p2);
criterion_main!(d21);