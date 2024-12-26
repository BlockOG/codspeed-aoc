use core::str;
use std::simd::{cmp::SimdPartialEq, u8x64, Mask};

unsafe fn part1_inner(input: &str) -> u64 {
    let mut input = input.as_ptr();
    let end = input.add(43 * 500);

    static mut KEY_TOP: [u64; 250] = [0; 250];
    static mut KEY_BOTTOM: [u64; 250] = [0; 250];
    let mut key_top_end = KEY_TOP.as_mut_ptr();
    let mut key_bottom_end = KEY_BOTTOM.as_mut_ptr();
    while input != end {
        let v = u8x64::load_select_ptr(
            input,
            Mask::from_bitmask(0b011111011111011111011111011111011111011111),
            u8x64::splat(0),
        )
        .simd_eq(u8x64::splat(b'#'))
        .to_bitmask();

        if v & 1 != 0 {
            *key_top_end = v;
            key_top_end = key_top_end.add(1);
        } else {
            *key_bottom_end = v;
            key_bottom_end = key_bottom_end.add(1);
        }

        input = input.add(43);
    }

    let mut res = 0;

    let mut key_top = KEY_TOP.as_ptr();
    while key_top != key_top_end {
        let mut key_bottom = KEY_BOTTOM.as_ptr();
        while key_bottom != key_bottom_end {
            res += ((*key_top & *key_bottom) == 0) as u64;

            key_bottom = key_bottom.add(1);
        }

        key_top = key_top.add(1);
    }

    res
}

pub fn part1(input: &str) -> u64 {
    unsafe { part1_inner(input) }
}
