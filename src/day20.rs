use core::str;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
unsafe fn part1_inner(input: &str) -> i32 {
    let start = memchr::memchr(b'S', input.as_bytes()).unwrap_unchecked();

    let input = input.as_ptr();

    const N: usize = 141;
    const M: usize = N + 1;
    const K: usize = N + 2;

    const fn calc_input_pos(x: usize, y: usize) -> usize {
        y * M + x
    }

    const fn calc_pos(x: usize, y: usize) -> usize {
        (y + 1) * K + x + 1
    }

    let mut path = [(0, 0); 10000];
    let mut path = path.as_mut_ptr();

    let mut distance = [0; K * K];
    let distance = distance.as_mut_ptr();

    let (mut x, mut y) = (start % M, start / M);
    let mut curr = path;
    let mut steps = 1;
    let mut last = 4;
    while *input.add(calc_input_pos(x, y)) != b'E' {
        *curr = (x, y);
        curr = curr.add(1);

        *distance.add(calc_pos(x, y)) = steps;

        if last != 0 && *input.add(calc_input_pos(x + 1, y)) != b'#' {
            x += 1;
            last = 1;
        } else if last != 1 && *input.add(calc_input_pos(x - 1, y)) != b'#' {
            x -= 1;
            last = 0;
        } else if last != 2 && *input.add(calc_input_pos(x, y + 1)) != b'#' {
            y += 1;
            last = 3;
        } else if last != 3 {
            y -= 1;
            last = 2;
        }

        steps += 1;
    }

    let mut res = 0;
    while path != curr {
        let (x, y) = *path;

        if *distance.add(calc_pos(x + 2, y)) != 0 {
            res += (*distance.add(calc_pos(x + 2, y)) > 100 + *distance.add(calc_pos(x, y))) as i32;
        }
        if *distance.add(calc_pos(x - 2, y)) != 0 {
            res += (*distance.add(calc_pos(x - 2, y)) > 100 + *distance.add(calc_pos(x, y))) as i32;
        }
        if *distance.add(calc_pos(x, y + 2)) != 0 {
            res += (*distance.add(calc_pos(x, y + 2)) > 100 + *distance.add(calc_pos(x, y))) as i32;
        }
        if *distance.add(calc_pos(x, y - 2)) != 0 {
            res += (*distance.add(calc_pos(x, y - 2)) >= 100 + 2 + *distance.add(calc_pos(x, y)))
                as i32;
        }

        path = path.add(1);
    }

    res
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
unsafe fn part2_inner(input: &str) -> i32 {
    let start = memchr::memchr(b'S', input.as_bytes()).unwrap_unchecked();

    let input = input.as_ptr();

    const N: usize = 141;
    const M: usize = N + 1;
    const K: usize = N + 38;

    const fn calc_input_pos(x: usize, y: usize) -> usize {
        y * M + x
    }

    const fn calc_pos(x: usize, y: usize) -> usize {
        (y + 19) * K + x + 19
    }

    let mut path = [(0, 0); 10000];
    let mut path = path.as_mut_ptr();

    let mut distance = [0; K * K];
    let distance = distance.as_mut_ptr();

    let (mut x, mut y) = (start % M, start / M);
    let mut curr = path;
    let mut steps = 1;
    let mut last = 4;
    while *input.add(calc_input_pos(x, y)) != b'E' {
        *curr = (x, y);
        curr = curr.add(1);

        *distance.add(calc_pos(x, y)) = steps;

        if last != 0 && *input.add(calc_input_pos(x + 1, y)) != b'#' {
            x += 1;
            last = 1;
        } else if last != 1 && *input.add(calc_input_pos(x - 1, y)) != b'#' {
            x -= 1;
            last = 0;
        } else if last != 2 && *input.add(calc_input_pos(x, y + 1)) != b'#' {
            y += 1;
            last = 3;
        } else if last != 3 {
            y -= 1;
            last = 2;
        }

        steps += 1;
    }

    *distance.add(calc_pos(x, y)) = steps;

    let mut res = 0;
    while path != curr {
        let (x, y) = *path;

        for dx in -20isize..=20 {
            for dy in dx.abs() - 20..=20 - dx.abs() {
                if *distance.add(calc_pos(x + dx as usize, y + dy as usize)) != 0 {
                    res += (*distance.add(calc_pos(x + dx as usize, y + dy as usize))
                        >= 100 + dx.abs() + dy.abs() + *distance.add(calc_pos(x, y)))
                        as i32;
                }
            }
        }

        path = path.add(1);
    }

    res
}

pub fn part1(input: &str) -> i32 {
    unsafe { part1_inner(input) }
}

pub fn part2(input: &str) -> i32 {
    unsafe { part2_inner(input) }
}
