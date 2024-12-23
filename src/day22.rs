use core::{slice, str};
use std::simd::{num::SimdUint, u32x32};

unsafe fn part1_inner(input: &str) -> u64 {
    let end = input.len();
    let mut input = input.as_ptr();
    let end = input.add(end);

    let mut res = 0;
    while input != end {
        let mut i = 0;
        i += (*input.add(0) as u32 - b'0' as u32) * 10000;
        i += (*input.add(1) as u32 - b'0' as u32) * 1000;
        i += (*input.add(2) as u32 - b'0' as u32) * 100;
        i += (*input.add(3) as u32 - b'0' as u32) * 10;
        i += (*input.add(4) as u32 - b'0' as u32) * 1;
        let c = *input.add(5) as u32;
        input = input.add(6);
        if c >= b'0' as u32 {
            i = i * 10 + c - b'0' as u32;
            let c = *input as u32;
            input = input.add(1);
            if c >= b'0' as u32 {
                i = i * 10 + c - b'0' as u32;
                let c = *input as u32;
                input = input.add(1);
                if c >= b'0' as u32 {
                    i = i * 10 + c - b'0' as u32;
                    input = input.add(1);
                }
            }
        }

        res += (u32x32::from_array([
            8685429, 5976613, 5736965, 1870103, 7162760, 5025953, 15873144, 12101663, 13031267,
            8752743, 14531921, 3977376, 1183600, 12205769, 5218631, 1112099, 14279190, 2387484,
            12244792, 13635808, 4302637, 11348105, 4531072, 6986829, 0, 0, 0, 0, 0, 0, 0, 0,
        ]) * ((u32x32::splat(i)
            >> u32x32::from_array([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 0, 0, 0, 0, 0, 0, 0, 0,
            ]))
            & u32x32::splat(1)))
        .reduce_xor() as u64;
    }

    res
}

unsafe fn part2_inner(input: &str) -> u32 {
    let end = input.len();
    let mut input = input.as_ptr();
    let end = input.add(end);

    let mut seq_map = [0; 19usize.pow(4)];
    let seq_map = seq_map.as_mut_ptr();
    let mut max_v = 0;

    let mut seen = [true; 130322];
    let seen = seen.as_mut_ptr();
    while input != end {
        for i in (0..130322).step_by(8) {
            *(seen.add(i) as *mut u64) = 0x0101010101010101;
        }

        let mut i = 0;
        i += (*input.add(0) as u32 - b'0' as u32) * 10000;
        i += (*input.add(1) as u32 - b'0' as u32) * 1000;
        i += (*input.add(2) as u32 - b'0' as u32) * 100;
        i += (*input.add(3) as u32 - b'0' as u32) * 10;
        i += (*input.add(4) as u32 - b'0' as u32) * 1;
        let c = *input.add(5) as u32;
        input = input.add(6);
        if c >= b'0' as u32 {
            i = i * 10 + c - b'0' as u32;
            let c = *input as u32;
            input = input.add(1);
            if c >= b'0' as u32 {
                i = i * 10 + c - b'0' as u32;
                let c = *input as u32;
                input = input.add(1);
                if c >= b'0' as u32 {
                    i = i * 10 + c - b'0' as u32;
                    input = input.add(1);
                }
            }
        }

        let mut last_i = i;
        i ^= (i << 6) & 0xffffff;
        i ^= i >> 5;
        i ^= (i << 11) & 0xffffff;
        let mut a = 9 + i % 10 - last_i % 10;

        last_i = i;
        i ^= (i << 6) & 0xffffff;
        i ^= i >> 5;
        i ^= (i << 11) & 0xffffff;
        let mut b = 9 + i % 10 - last_i % 10;

        last_i = i;
        i ^= (i << 6) & 0xffffff;
        i ^= i >> 5;
        i ^= (i << 11) & 0xffffff;
        let mut c = 9 + i % 10 - last_i % 10;

        last_i = i;
        i ^= (i << 6) & 0xffffff;
        i ^= i >> 5;
        i ^= (i << 11) & 0xffffff;
        let mut d = 9 + i % 10 - last_i % 10;

        for _ in 0..2000 - 4 {
            let v = (a * 19 * 19 * 19 + b * 19 * 19 + c * 19 + d) as usize;
            *seq_map.add(v) += (i % 10) * *seen.add(v) as u32;
            max_v = max_v.max(*seq_map.add(v));
            *seen.add(v) = false;

            last_i = i;
            i ^= (i << 6) & 0xffffff;
            i ^= i >> 5;
            i ^= (i << 11) & 0xffffff;
            a = b;
            b = c;
            c = d;
            d = 9 + i % 10 - last_i % 10;
        }
    }

    max_v
}

pub fn part1(input: &str) -> u64 {
    unsafe { part1_inner(input) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input) }
}
