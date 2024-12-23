// use std::hint::unreachable_unchecked;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Nothing = 0,
    Wall = 1,
    Object = 2,
}

pub fn part1(_input: &str) -> usize {
    // unsafe {
    // let n = memchr::memchr(b'\n', input.as_bytes()).unwrap();
    // let (mut x, mut y) = (0, 0);
    // let mut a: Vec<Tile> = vec![Tile::Nothing; n * n];
    // for (i, v) in input[..n * (n + 1) - 1].lines().enumerate() {
    //     for (j, v) in v.bytes().enumerate() {
    //         match v {
    //             b'.' => {}
    //             b'@' => {
    //                 y = i;
    //                 x = j * 2;
    //             }
    //             b'#' => {
    //                 *(a.as_mut_ptr() as *mut u8).add(i * n + j) = 0x01;
    //             }
    //             b'O' => {
    //                 *(a.as_mut_ptr() as *mut u8).add(i * n + j) = 0x02;
    //             }
    //             _ => unreachable_unchecked(),
    //         }
    //     }
    // }

    // for i in b.bytes() {
    //     match i {
    //         b'^' => {
    //             let mut cy = y - 1;
    //             while a[cy * n + x] == Tile::Object {
    //                 cy -= 1;
    //             }
    //             if a[cy * n + x] == Tile::Wall {
    //                 continue;
    //             }
    //             a[cy * n + x] = Tile::Object;
    //             y -= 1;
    //             a[y * n + x] = Tile::Nothing;
    //         }
    //         b'<' => {
    //             let mut cx = x - 1;
    //             while a[y * n + cx] == Tile::Object {
    //                 cx -= 1;
    //             }
    //             if a[y * n + cx] == Tile::Wall {
    //                 continue;
    //             }
    //             a[y * n + cx] = Tile::Object;
    //             x -= 1;
    //             a[y * n + x] = Tile::Nothing;
    //         }
    //         b'v' => {
    //             let mut cy = y + 1;
    //             while a[cy * n + x] == Tile::Object {
    //                 cy += 1;
    //             }
    //             if a[cy * n + x] == Tile::Wall {
    //                 continue;
    //             }
    //             a[cy * n + x] = Tile::Object;
    //             y += 1;
    //             a[y * n + x] = Tile::Nothing;
    //         }
    //         b'>' => {
    //             let mut cx = x + 1;
    //             while a[y * n + cx] == Tile::Object {
    //                 cx += 1;
    //             }
    //             if a[y * n + cx] == Tile::Wall {
    //                 continue;
    //             }
    //             a[y * n + cx] = Tile::Object;
    //             x += 1;
    //             a[y * n + x] = Tile::Nothing;
    //         }
    //         _ => {}
    //     }
    // }

    // a.into_iter()
    //     .enumerate()
    //     .map(|(i, v)| {
    //         v.into_iter()
    //             .enumerate()
    //             .filter_map(|(j, v)| (v == Tile::Object).then(|| i * 100 + j))
    //             .sum::<usize>()
    //     })
    //     .sum::<usize>()
    0
    // }
}

pub fn part2(input: &str) -> usize {
    unsafe {
        const N: usize = 50;
        const M: usize = N * 2;

        let mut pos = 0;
        let mut a = [Tile::Nothing; N * M];
        let mut i = 0;
        for v in input.get_unchecked(..N * (N + 1) - 1).bytes() {
            match v {
                b'.' => {}
                b'@' => {
                    pos = i;
                }
                b'#' => {
                    *(a.as_mut_ptr() as *mut u16).add(i) = 0x0101;
                }
                b'O' => {
                    *(a.as_mut_ptr() as *mut u16).add(i) = 0x0002;
                }
                b'\n' => i -= 1,
                _ => unreachable!(),
            }
            i += 1;
        }

        unsafe fn can_push<const D: usize>(pos: usize, a: &[Tile; N * M]) -> bool {
            match a.get_unchecked(pos) {
                Tile::Nothing => {
                    if *a.get_unchecked(pos - 1) == Tile::Object {
                        can_push::<D>(pos - 1, a)
                    } else {
                        true
                    }
                }
                Tile::Wall => false,
                Tile::Object => match a.get_unchecked(pos + M * D) {
                    Tile::Wall => false,
                    _ => can_push::<D>(pos + M * D, a) && can_push::<D>(pos + M * D + 1, a),
                },
            }
        }

        unsafe fn push_vert<const D: usize>(pos: usize, a: &mut [Tile; N * M]) {
            match a.get_unchecked(pos) {
                Tile::Nothing => {
                    if *a.get_unchecked(pos - 1) == Tile::Object {
                        push_vert::<D>(pos - 1, a);
                    }
                }
                Tile::Wall => unreachable!(),
                Tile::Object => {
                    match a.get_unchecked(pos + M * D) {
                        Tile::Wall => unreachable!(),
                        _ => {
                            push_vert::<D>(pos + M * D, a);
                            push_vert::<D>(pos + M * D + 1, a);
                        }
                    }
                    *a.get_unchecked_mut(pos + M * D) = *a.get_unchecked(pos);
                    *a.get_unchecked_mut(pos) = Tile::Nothing;
                }
            }
        }

        for b in input.get_unchecked(N * (N + 1) + 1..).bytes() {
            match b {
                b'^' => {
                    if can_push::<{ usize::MAX }>(pos - M, &a) {
                        push_vert::<{ usize::MAX }>(pos - M, &mut a);
                        pos -= M;
                    }
                }
                b'<' => {
                    let mut cp = pos - 1;
                    while a[cp - 1] == Tile::Object {
                        cp -= 2;
                    }
                    if a[cp] == Tile::Wall {
                        continue;
                    }
                    pos -= 1;
                    for cp in (cp..pos).step_by(2) {
                        *a.get_unchecked_mut(cp) = Tile::Object;
                        *a.get_unchecked_mut(cp + 1) = Tile::Nothing;
                    }
                    a[pos] = Tile::Nothing;
                }
                b'v' => {
                    if can_push::<1>(pos + M, &a) {
                        push_vert::<1>(pos + M, &mut a);
                        pos += M;
                    }
                }
                b'>' => {
                    let mut cp = pos + 1;
                    while a[cp] == Tile::Object {
                        cp += 2;
                    }
                    if a[cp] == Tile::Wall {
                        continue;
                    }
                    pos += 1;
                    for cp in (pos..cp).step_by(2) {
                        *a.get_unchecked_mut(cp + 1) = Tile::Object;
                        *a.get_unchecked_mut(cp + 2) = Tile::Nothing;
                    }
                    a[pos] = Tile::Nothing;
                }
                _ => continue,
            }

            // for (i, v) in a.iter().enumerate() {
            //     if i % m == x && i / m == y {
            //         print!("{}", b as char);
            //     } else {
            //         print!(
            //             "{}",
            //             match v {
            //                 Tile::Nothing => {
            //                     if a[i - 1] == Tile::Object {
            //                         ']'
            //                     } else {
            //                         '.'
            //                     }
            //                 }
            //                 Tile::Wall => '#',
            //                 Tile::Object => '[',
            //             }
            //         );
            //     }

            //     if (i + 1) % m == 0 {
            //         println!();
            //     }
            // }
            // println!();
            // stdin().read_line(&mut String::new()).unwrap();
        }

        a.into_iter()
            .enumerate()
            .filter_map(|(i, v)| (v == Tile::Object).then(|| (i / M) * 100 + i % M))
            .sum::<usize>()
    }
}
