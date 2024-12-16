#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Nothing = 0,
    Wall = 1,
    Object = 2,
}

pub fn part1(input: &str) -> usize {
    let (a, b) = input.split_once("\n\n").unwrap();
    let (mut x, mut y) = (0, 0);
    let mut a: Vec<Vec<_>> = a
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.bytes()
                .enumerate()
                .map(|(j, b)| match b {
                    b'.' => Tile::Nothing,
                    b'@' => {
                        y = i;
                        x = j;
                        Tile::Nothing
                    }
                    b'#' => Tile::Wall,
                    b'O' => Tile::Object,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    for i in b.bytes() {
        match i {
            b'^' => {
                let mut cy = y - 1;
                while a[cy][x] == Tile::Object {
                    cy -= 1;
                }
                if a[cy][x] == Tile::Wall {
                    continue;
                }
                a[cy][x] = Tile::Object;
                y -= 1;
                a[y][x] = Tile::Nothing;
            }
            b'<' => {
                let mut cx = x - 1;
                while a[y][cx] == Tile::Object {
                    cx -= 1;
                }
                if a[y][cx] == Tile::Wall {
                    continue;
                }
                a[y][cx] = Tile::Object;
                x -= 1;
                a[y][x] = Tile::Nothing;
            }
            b'v' => {
                let mut cy = y + 1;
                while a[cy][x] == Tile::Object {
                    cy += 1;
                }
                if a[cy][x] == Tile::Wall {
                    continue;
                }
                a[cy][x] = Tile::Object;
                y += 1;
                a[y][x] = Tile::Nothing;
            }
            b'>' => {
                let mut cx = x + 1;
                while a[y][cx] == Tile::Object {
                    cx += 1;
                }
                if a[y][cx] == Tile::Wall {
                    continue;
                }
                a[y][cx] = Tile::Object;
                x += 1;
                a[y][x] = Tile::Nothing;
            }
            _ => {}
        }
    }

    a.into_iter()
        .enumerate()
        .map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .filter_map(|(j, v)| (v == Tile::Object).then(|| i * 100 + j))
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    unsafe {
        let n = memchr::memchr(b'\n', input.as_bytes()).unwrap();
        let m = n * 2;
        let (mut x, mut y) = (0, 0);
        let mut a: Vec<Tile> = vec![Tile::Nothing; n * m];
        for (i, v) in input[..n * (n + 1) - 1].lines().enumerate() {
            for (j, v) in v.bytes().enumerate() {
                match v {
                    b'.' => {}
                    b'@' => {
                        y = i;
                        x = j * 2;
                    }
                    b'#' => {
                        *(a.as_mut_ptr() as *mut u16).add(i * n + j) = 0x0101;
                    }
                    b'O' => {
                        *(a.as_mut_ptr() as *mut u16).add(i * n + j) = 0x0002;
                    }
                    _ => unreachable!(),
                }
            }
        }

        fn can_push<const D: usize>(x: usize, y: usize, a: &Vec<Tile>, m: usize) -> bool {
            match a[y * m + x] {
                Tile::Nothing => {
                    if a[y * m + x - 1] == Tile::Object {
                        can_push::<D>(x - 1, y, a, m)
                    } else {
                        true
                    }
                }
                Tile::Wall => false,
                Tile::Object => match a[(y + D) * m + x] {
                    Tile::Wall => false,
                    _ => can_push::<D>(x, y + D, a, m) && can_push::<D>(x + 1, y + D, a, m),
                },
            }
        }

        fn push_vert<const D: usize>(x: usize, y: usize, a: &mut Vec<Tile>, m: usize) {
            match a[y * m + x] {
                Tile::Nothing => {
                    if a[y * m + x - 1] == Tile::Object {
                        push_vert::<D>(x - 1, y, a, m);
                    }
                }
                Tile::Wall => unreachable!(),
                Tile::Object => {
                    match a[(y + D) * m + x] {
                        Tile::Wall => unreachable!(),
                        _ => {
                            push_vert::<D>(x, y + D, a, m);
                            push_vert::<D>(x + 1, y + D, a, m);
                        }
                    }
                    a[(y + D) * m + x] = a[y * m + x];
                    a[y * m + x] = Tile::Nothing;
                }
            }
        }

        for b in input[n * (n + 1) + 1..].bytes() {
            match b {
                b'^' => {
                    if can_push::<{ usize::MAX }>(x, y - 1, &a, m) {
                        push_vert::<{ usize::MAX }>(x, y - 1, &mut a, m);
                        y -= 1;
                    }
                }
                b'<' => {
                    let mut cx = x - 1;
                    while a[y * m + cx - 1] == Tile::Object {
                        cx -= 2;
                    }
                    if a[y * m + cx] == Tile::Wall {
                        continue;
                    }
                    x -= 1;
                    for cx in (cx..x).step_by(2) {
                        a[y * m + cx] = Tile::Object;
                        a[y * m + cx + 1] = Tile::Nothing;
                    }
                    a[y * m + x] = Tile::Nothing;
                }
                b'v' => {
                    if can_push::<1>(x, y + 1, &a, m) {
                        push_vert::<1>(x, y + 1, &mut a, m);
                        y += 1;
                    }
                }
                b'>' => {
                    let mut cx = x + 1;
                    while a[y * m + cx] == Tile::Object {
                        cx += 2;
                    }
                    if a[y * m + cx] == Tile::Wall {
                        continue;
                    }
                    x += 1;
                    for cx in (x..cx).step_by(2) {
                        a[y * m + cx + 1] = Tile::Object;
                        a[y * m + cx + 2] = Tile::Nothing;
                    }
                    a[y * m + x] = Tile::Nothing;
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
            .filter_map(|(i, v)| (v == Tile::Object).then(|| (i / m) * 100 + i % m))
            .sum::<usize>()
    }
}
