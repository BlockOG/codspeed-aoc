use core::str;

use std::{arch::x86_64::_pext_u32, hint::unreachable_unchecked};

unsafe fn get_value(v: u32, values: *const bool, gates: *const (u32, u32, u8)) -> bool {
    if ((v & 0b11111) - 0b11010) >= 0u32.wrapping_sub(2) {
        return *values.add(v as usize);
    }

    let (a, b, op) = *gates.add(v as usize);
    match op {
        b'A' => get_value(b, values, gates) && get_value(a, values, gates),
        b'O' => get_value(b, values, gates) || get_value(a, values, gates),
        b'X' => get_value(a, values, gates) != get_value(b, values, gates),
        _ => unreachable_unchecked(),
    }
}

fn pext_u32(v: u32, m: u32) -> u32 {
    unsafe { _pext_u32(v, m) }
}

unsafe fn hash_3cc(cc: *const u8) -> u32 {
    pext_u32(*(cc as *const u32), 0b00011111_00011111_00011111)
}

unsafe fn part1_inner(input: &str) -> u64 {
    let mut input = input.as_ptr();

    static mut VALUES: [bool; 1 << 15] = [false; 1 << 15];
    let values = VALUES.as_mut_ptr();
    for l in (0..7 * 2 * 45).step_by(7) {
        *values.add(hash_3cc(input.add(l)) as usize) = *input.add(l + 5) == b'1';
    }

    input = input.add(631);

    static mut GATES: [(u32, u32, u8); 1 << 15] = [(0, 0, 0); 1 << 15];
    let gates = GATES.as_mut_ptr();
    for _ in 0..222 {
        let a = hash_3cc(input);
        let i = (*input.add(4) != b'O') as usize;
        let b = hash_3cc(input.add(7 + i));
        let c = hash_3cc(input.add(14 + i));

        *gates.add(c as usize) = (a, b, *input.add(4));

        input = input.add(18 + i);
    }

    let mut res = 0;
    //                 5---- 4---- z----
    let mut i = 0b10101_10100_11010; // z45
    for _ in 0..46 {
        res = (res << 1) | get_value(i, values, gates) as u64;
        if i & 0b1111_00000_00000 == 0 {
            i -= 0b1_00000u32.wrapping_sub(0b1001_00000_00000);
        } else {
            i -= 0b1_00000_00000;
        }
    }

    res
}

unsafe fn part2_inner(input: &str) -> u32 {
    0
}

pub fn part1(input: &str) -> u64 {
    unsafe { part1_inner(input) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { part2_inner(input) }
}
