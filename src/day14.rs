use std::mem::MaybeUninit;

use crate::UncheckedArithmetic;

pub fn part1(input: &str) -> i32 {
    unsafe {
        let end = input.len();
        let mut input = input.as_ptr();
        let end = input.add(end);

        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        while input < end {
            let mut x = (*input.add(2) as i32).usub(b'0' as i32);
            let c = *input.add(3) as i32;
            input = input.add(4);
            if c >= b'0' as i32 {
                x = x.umul(10).uadd(c.usub(b'0' as i32));
                let c = *input as i32;
                input = input.add(1);
                if c >= b'0' as i32 {
                    x = x.umul(10).uadd(c.usub(b'0' as i32));
                    input = input.add(1);
                }
            }

            let mut y = (*input as i32).usub(b'0' as i32);
            let c = *input.add(1) as i32;
            input = input.add(2);
            if c >= b'0' as i32 {
                y = y.umul(10).uadd(c.usub(b'0' as i32));
                let c = *input as i32;
                input = input.add(1);
                if c >= b'0' as i32 {
                    y = y.umul(10).uadd(c.usub(b'0' as i32));
                    input = input.add(1);
                }
            }

            input = input.add(2);
            let mut dx;
            if *input == b'-' {
                dx = (b'0' as i32).usub(*input.add(1) as i32).umul(100);
                let c = *input.add(2) as i32;
                input = input.add(3);
                if c >= b'0' as i32 {
                    dx = dx.umul(10).usub(c.usub(b'0' as i32).umul(100));
                    input = input.add(1);
                }
            } else {
                dx = ((*input - b'0') as i32).umul(100);
                let c = *input.add(1) as i32;
                input = input.add(2);
                if c >= b'0' as i32 {
                    dx = dx.umul(10).uadd(c.usub(b'0' as i32).umul(100));
                    input = input.add(1);
                }
            };

            let mut dy;
            if *input == b'-' {
                dy = (b'0' as i32).usub(*input.add(1) as i32).umul(100);
                let c = *input.add(2) as i32;
                input = input.add(3);
                if c >= b'0' as i32 {
                    dy = dy.umul(10).usub(c.usub(b'0' as i32).umul(100));
                    input = input.add(1);
                }
            } else {
                dy = ((*input - b'0') as i32).umul(100);
                let c = *input.add(1) as i32;
                input = input.add(2);
                if c >= b'0' as i32 {
                    dy = dy.umul(10).uadd(c.usub(b'0' as i32).umul(100));
                    input = input.add(1);
                }
            };

            let rx = x.uadd(dx).urem(101);
            if 0 <= rx && rx < 50 || rx < -51 {
                let ry = y.uadd(dy).urem(103);
                if 0 <= ry && ry < 51 || ry < -52 {
                    q1 += 1;
                } else if 51 < ry || -52 < ry && ry < 0 {
                    q2 += 1;
                }
            } else if 50 < rx || -51 < rx && rx < 0 {
                let ry = y.uadd(dy).urem(103);
                if 0 <= ry && ry < 51 || ry < -52 {
                    q3 += 1;
                } else if 51 < ry || -52 < ry && ry < 0 {
                    q4 += 1;
                }
            }
        }

        q1 * q2 * q3 * q4
    }
}

pub fn part2(input: &str) -> i32 {
    unsafe {
        let end = input.len();
        let mut input = input.as_ptr();
        let end = input.add(end);

        let mut robots = Vec::with_capacity(500);
        while input < end {
            let mut x = (*input.add(2) as i32).usub(b'0' as i32);
            let c = *input.add(3) as i32;
            input = input.add(4);
            if c >= b'0' as i32 {
                x = x.umul(10).uadd(c.usub(b'0' as i32));
                let c = *input as i32;
                input = input.add(1);
                if c >= b'0' as i32 {
                    x = x.umul(10).uadd(c.usub(b'0' as i32));
                    input = input.add(1);
                }
            }

            let mut y = (*input as i32).usub(b'0' as i32);
            let c = *input.add(1) as i32;
            input = input.add(2);
            if c >= b'0' as i32 {
                y = y.umul(10).uadd(c.usub(b'0' as i32));
                let c = *input as i32;
                input = input.add(1);
                if c >= b'0' as i32 {
                    y = y.umul(10).uadd(c.usub(b'0' as i32));
                    input = input.add(1);
                }
            }

            input = input.add(2);
            let mut dx;
            if *input == b'-' {
                dx = (b'0' as i32).usub(*input.add(1) as i32);
                let c = *input.add(2) as i32;
                input = input.add(3);
                if c >= b'0' as i32 {
                    dx = dx.umul(10).usub(c.usub(b'0' as i32));
                    input = input.add(1);
                }

                dx += 101;
            } else {
                dx = (*input - b'0') as i32;
                let c = *input.add(1) as i32;
                input = input.add(2);
                if c >= b'0' as i32 {
                    dx = dx.umul(10).uadd(c.usub(b'0' as i32));
                    input = input.add(1);
                }
            };

            let mut dy;
            if *input == b'-' {
                dy = (b'0' as i32).usub(*input.add(1) as i32);
                let c = *input.add(2) as i32;
                input = input.add(3);
                if c >= b'0' as i32 {
                    dy = dy.umul(10).usub(c.usub(b'0' as i32));
                    input = input.add(1);
                }

                dy += 103;
            } else {
                dy = (*input - b'0') as i32;
                let c = *input.add(1) as i32;
                input = input.add(2);
                if c >= b'0' as i32 {
                    dy = dy.umul(10).uadd(c.usub(b'0' as i32));
                    input = input.add(1);
                }
            };

            robots.push([x, y, dx, dy]);
        }

        let mut lineup_vertical = 0;
        let mut lineup_horizontal = 0;

        let mut rows = MaybeUninit::array_assume_init([MaybeUninit::uninit(); 103]);
        let mut columns = MaybeUninit::array_assume_init([MaybeUninit::uninit(); 101]);
        'outer: for i in 1..=103 {
            rows.fill(0);
            columns.fill(0);

            for &[x, y, dx, dy] in robots.iter() {
                let x = x.uadd(dx.umul(i)).urem(101);
                let y = y.uadd(dy.umul(i)).urem(103);

                rows[y as usize] += 1;
                columns[x as usize] += 1;

                if rows[y as usize] >= 31 {
                    lineup_horizontal = i;
                    for i in i + 1..=103 {
                        columns.fill(0);

                        for &[x, _, dx, _] in robots.iter() {
                            let x = x.uadd(dx.umul(i)).urem(101);

                            columns[x as usize] += 1;
                            if columns[x as usize] >= 33 {
                                lineup_vertical = i;
                                break 'outer;
                            }
                        }
                    }
                }

                if columns[x as usize] >= 33 {
                    lineup_horizontal = i;
                    for i in i + 1..=103 {
                        rows.fill(0);

                        for &[_, y, _, dy] in robots.iter() {
                            let y = y.uadd(dy.umul(i)).urem(103);

                            rows[y as usize] += 1;
                            if rows[y as usize] >= 31 {
                                lineup_vertical = i;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        (lineup_horizontal * 101 * 51 - lineup_vertical * 103 * 50).rem_euclid(101 * 103)
    }
}
