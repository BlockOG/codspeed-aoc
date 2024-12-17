use core::str;
use std::mem::transmute;

pub fn part1(input: &str) -> &str {
    unsafe {
        let input = input.as_ptr();

        let mut a = 0;
        a += 10000000 * (*input.add(12) as u64 - b'0' as u64);
        a += 1000000 * (*input.add(13) as u64 - b'0' as u64);
        a += 100000 * (*input.add(14) as u64 - b'0' as u64);
        a += 10000 * (*input.add(15) as u64 - b'0' as u64);
        a += 1000 * (*input.add(16) as u64 - b'0' as u64);
        a += 100 * (*input.add(17) as u64 - b'0' as u64);
        a += 10 * (*input.add(18) as u64 - b'0' as u64);
        a += 1 * (*input.add(19) as u64 - b'0' as u64);

        let xx = *input.add(65) as u64 - b'0' as u64;
        let x = xx
            ^ if *input.add(71) == b'1' {
                *input.add(73) as u64 - b'0' as u64
            } else if *input.add(75) == b'1' {
                *input.add(77) as u64 - b'0' as u64
            } else {
                *input.add(81) as u64 - b'0' as u64
            };

        static mut RES: [u8; 18] = [0; 18];
        let mut res_ptr = RES.as_mut_ptr();
        while a != 0 {
            let b = x ^ a ^ (a >> ((a & 7) ^ xx));

            *res_ptr = b',';
            *res_ptr.add(1) = b'0' + (b & 7) as u8;
            res_ptr = res_ptr.add(2);
            a >>= 3;
        }

        str::from_raw_parts(RES.as_ptr().add(1), 17)
    }
}

pub fn part2(input: &str) -> u64 {
    unsafe {
        const LUT: [[[[[[u64; 8]; 8]; 6]; 5]; 5]; 8] =
            unsafe { transmute(*include_bytes!("../day17lut")) };

        let input = input.as_ptr();

        let xx = *input.add(65) as u64 - b'0' as u64;
        let x = if *input.add(71) == b'1' {
            *input.add(73) as u64 - b'0' as u64
        } else if *input.add(75) == b'1' {
            *input.add(77) as u64 - b'0' as u64
        } else {
            *input.add(81) as u64 - b'0' as u64
        };
        let dx = if *input.add(71) == b'4' {
            *input.add(73) as u64 - b'0' as u64
        } else if *input.add(75) == b'4' {
            *input.add(77) as u64 - b'0' as u64
        } else {
            *input.add(81) as u64 - b'0' as u64
        };

        let f1 = *input.add(71) as u64 - b'0' as u64;
        let f2 = *input.add(75) as u64 - b'0' as u64;
        let f3 = *input.add(79) as u64 - b'0' as u64;

        LUT[xx as usize][f1 as usize][f2 as usize][f3 as usize][x as usize][dx as usize]
    }
}
