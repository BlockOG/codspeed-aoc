#![allow(internal_features)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(core_intrinsics)]
#![feature(maybe_uninit_array_assume_init)]

use std::intrinsics;

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

#[allow(unused)]
trait UncheckedArithmetic {
    fn uadd(self, other: Self) -> Self;
    fn usub(self, other: Self) -> Self;
    fn umul(self, other: Self) -> Self;
    fn udiv(self, other: Self) -> Self;
    fn urem(self, other: Self) -> Self;
}

macro_rules! impl_unchecked_arithmetic {
    ($($ty:ty),+) => {
        $(
            impl UncheckedArithmetic for $ty {
                fn uadd(self, other: Self) -> Self {
                    unsafe { intrinsics::unchecked_add(self, other) }
                }
                fn usub(self, other: Self) -> Self {
                    unsafe { intrinsics::unchecked_sub(self, other) }
                }
                fn umul(self, other: Self) -> Self {
                    unsafe { intrinsics::unchecked_mul(self, other) }
                }
                fn udiv(self, other: Self) -> Self {
                    unsafe { intrinsics::unchecked_div(self, other) }
                }
                fn urem(self, other: Self) -> Self {
                    unsafe { intrinsics::unchecked_rem(self, other) }
                }
            }
        )+
    };
}

impl_unchecked_arithmetic!(u8, u16, u32, u64, u128, usize);
impl_unchecked_arithmetic!(i8, i16, i32, i64, i128, isize);

pub mod day13;
pub mod day14;
pub mod day15;

aoc_lib! { year = 2024 }
