use core::str;
use std::mem::transmute;

macro_rules! parse_num {
    ($input:ident, $i:literal) => {{
        let mut x = 0;
        x += 100 * (*$input.add($i + 0) as usize - b'0' as usize);
        x += 10 * (*$input.add($i + 1) as usize - b'0' as usize);
        x += 1 * (*$input.add($i + 2) as usize - b'0' as usize);
        x
    }};
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
unsafe fn part1_inner(input: &str) -> u64 {
    let input = input.as_ptr();

    const LUT: [u64; 1000] = unsafe { transmute(*include_bytes!("../day21p1lut")) };

    let p1 = parse_num!(input, 0);
    let p2 = parse_num!(input, 5);
    let p3 = parse_num!(input, 10);
    let p4 = parse_num!(input, 15);
    let p5 = parse_num!(input, 20);

    LUT[p1] + LUT[p2] + LUT[p3] + LUT[p4] + LUT[p5]
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
unsafe fn part2_inner(input: &str) -> u64 {
    let input = input.as_ptr();

    const LUT: [u64; 1000] = unsafe { transmute(*include_bytes!("../day21p2lut")) };

    let p1 = parse_num!(input, 0);
    let p2 = parse_num!(input, 5);
    let p3 = parse_num!(input, 10);
    let p4 = parse_num!(input, 15);
    let p5 = parse_num!(input, 20);

    LUT[p1] + LUT[p2] + LUT[p3] + LUT[p4] + LUT[p5]
}

pub fn part1(input: &str) -> u64 {
    unsafe { part1_inner(input) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { part2_inner(input) }
}
