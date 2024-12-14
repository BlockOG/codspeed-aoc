#![allow(internal_features)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(core_intrinsics)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day13;

aoc_lib! { year = 2024 }
