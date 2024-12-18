use core::{slice, str};
use std::{
    mem::transmute,
    simd::{cmp::SimdPartialEq, u8x16},
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

        static mut RES: [u8; 18] = [0; 18];
        let res_ptr = RES.as_mut_ptr();

        macro_rules! do_output {
            ($a:ident, $x:ident, $xx:ident, $res_ptr:ident, $c:literal) => {
                let a = ($a >> 3 * $c) as u16;
                *($res_ptr as *mut u16).add($c) =
                    (b',' as u16) << 8 | (b'0' as u16 | (($x ^ a ^ a >> (a & 7 ^ $xx)) & 7));
            };
        }

        do_output!(a, x, xx, res_ptr, 0);
        do_output!(a, x, xx, res_ptr, 1);
        do_output!(a, x, xx, res_ptr, 2);
        do_output!(a, x, xx, res_ptr, 3);
        do_output!(a, x, xx, res_ptr, 4);
        do_output!(a, x, xx, res_ptr, 5);
        do_output!(a, x, xx, res_ptr, 6);
        do_output!(a, x, xx, res_ptr, 7);
        do_output!(a, x, xx, res_ptr, 8);

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
