use core::{slice, str};
use std::{
    mem::transmute,
    simd::{cmp::SimdPartialEq, num::SimdUint, u16x8, u32x8, u8x16, Mask},
};

pub fn part1(input: &str) -> &str {
    unsafe {
        let input = input.as_ptr();

        let mut a = 0;
        a += 10000000 * (*input.add(12) as u32 - b'0' as u32);
        a += 1000000 * (*input.add(13) as u32 - b'0' as u32);
        a += 100000 * (*input.add(14) as u32 - b'0' as u32);
        a += 10000 * (*input.add(15) as u32 - b'0' as u32);
        a += 1000 * (*input.add(16) as u32 - b'0' as u32);
        a += 100 * (*input.add(17) as u32 - b'0' as u32);
        a += 10 * (*input.add(18) as u32 - b'0' as u32);
        a += 1 * (*input.add(19) as u32 - b'0' as u32);

        let xx = *input.add(65) as u16 - b'0' as u16;

        let find_x = u8x16::from_slice(slice::from_raw_parts(input.add(71), 16));

        let find_x_mask = find_x.simd_eq(u8x16::from_array([
            b'1', 0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0, 0, 0, 0, 0,
        ]));

        let x = xx
            ^ (*input.add(71 + 2 + find_x_mask.first_set().unwrap_unchecked()) as u16
                - b'0' as u16);

        static mut RES: [u8; 17] = [0; 17];
        let res_ptr = RES.as_mut_ptr();

        let simd = (u32x8::splat(a)
            >> u32x8::from_array([3 * 0, 3 * 1, 3 * 2, 3 * 3, 3 * 4, 3 * 5, 3 * 6, 3 * 7]))
        .cast::<u16>();
        let simd = (u16x8::splat(x) ^ simd ^ simd >> (simd & u16x8::splat(7) ^ u16x8::splat(xx)))
            & u16x8::splat(7);
        (simd | u16x8::splat((b',' as u16) << 8 | b'0' as u16))
            .store_select_ptr(res_ptr as *mut u16, Mask::splat(true));

        let a = (a >> 3 * 8) as u16;
        *res_ptr.add(16) = (b'0' as u16 | ((x ^ a ^ a >> (a & 7 ^ xx)) & 7)) as u8;

        str::from_raw_parts(res_ptr, 17)
    }
}

pub fn part2(input: &str) -> u64 {
    unsafe {
        const LUT: [u64; 8 * 8 * 8 * 8 * 6 * 5 * 5] =
            unsafe { transmute(*include_bytes!("../day17lut")) };

        let input = input.as_ptr();

        let xx = *input.add(65) as usize - b'0' as usize;

        let f1 = *input.add(71) as usize - b'0' as usize;
        let o1 = *input.add(73) as usize - b'0' as usize;

        let f2 = *input.add(75) as usize - b'0' as usize;
        let o2 = *input.add(77) as usize - b'0' as usize;

        let f3 = *input.add(79) as usize - b'0' as usize;
        let o3 = *input.add(81) as usize - b'0' as usize;

        LUT[f1 * 8 * 8 * 8 * 8 * 6 * 5
            + f2 * 8 * 8 * 8 * 8 * 6
            + f3 * 8 * 8 * 8 * 8
            + o1 * 8 * 8 * 8
            + o2 * 8 * 8
            + o3 * 8
            + xx]
    }
}
