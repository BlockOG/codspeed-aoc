use core::str;
use std::hint::unreachable_unchecked;

pub fn part1(input: &str) -> u32 {
    unsafe {
        let mut input = input.as_ptr();

        const N: usize = 73;

        #[inline(always)]
        const fn calc_pos(x: usize, y: usize) -> usize {
            (y + 1) * N + x + 1
        }

        let mut grid = const {
            let mut grid = [false; N * N];

            let mut i = 0;
            while i < N {
                grid[i] = true;
                grid[i * N] = true;
                grid[N * N - i - 1] = true;
                grid[N * N - i * N - 1] = true;
                i += 1;
            }

            grid[calc_pos(0, 0)] = true;

            grid
        };
        let grid = grid.as_mut_ptr();
        for _ in 0..1024 {
            let mut x = *input as usize - b'0' as usize;
            let c = *input.add(1) as usize;
            input = input.add(2);
            if c >= b'0' as usize {
                x = x * 10 + (c - b'0' as usize);
                input = input.add(1);
            }

            let mut y = *input as usize - b'0' as usize;
            let c = *input.add(1) as usize;
            input = input.add(2);
            if c >= b'0' as usize {
                y = y * 10 + (c - b'0' as usize);
                input = input.add(1);
            }

            *grid.add(calc_pos(x, y)) = true;
        }

        let mut q = [calc_pos(0, 0); 71 * 71];
        let mut c = q.as_mut_ptr();
        let mut e = q.as_mut_ptr().add(1);
        for cost in 0.. {
            for _ in 0..e.offset_from(c) {
                let pos = *c;

                if pos == calc_pos(70, 70) {
                    return cost;
                }
                c = c.add(1);

                if !*grid.add(pos + N) {
                    *e = pos + N;
                    e = e.add(1);
                    *grid.add(pos + N) = true;
                }
                if !*grid.add(pos + 1) {
                    *e = pos + 1;
                    e = e.add(1);
                    *grid.add(pos + 1) = true;
                }
                if !*grid.add(pos - N) {
                    *e = pos - N;
                    e = e.add(1);
                    *grid.add(pos - N) = true;
                }
                if !*grid.add(pos - 1) {
                    *e = pos - 1;
                    e = e.add(1);
                    *grid.add(pos - 1) = true;
                }
            }
        }

        unreachable_unchecked()
    }
}

pub fn part2(input: &str) -> &str {
    unsafe {
        let end = input.len();
        let mut input = input.as_ptr();
        let start = input;
        let end = input.add(end);

        const N: usize = 73;

        #[inline(always)]
        const fn calc_pos(x: usize, y: usize) -> usize {
            (y + 1) * N + x + 1
        }

        let mut grid = const {
            let mut grid = [false; N * N];

            let mut i = 0;
            while i < N {
                grid[i] = true;
                grid[i * N] = true;
                grid[N * N - i - 1] = true;
                grid[N * N - i * N - 1] = true;
                i += 1;
            }

            grid
        };
        let grid = grid.as_mut_ptr();
        while input < end {
            let mut x = *input as usize - b'0' as usize;
            let c = *input.add(1) as usize;
            input = input.add(2);
            if c >= b'0' as usize {
                x = x * 10 + (c - b'0' as usize);
                input = input.add(1);
            }

            let mut y = *input as usize - b'0' as usize;
            let c = *input.add(1) as usize;
            input = input.add(2);
            if c >= b'0' as usize {
                y = y * 10 + (c - b'0' as usize);
                input = input.add(1);
            }

            *grid.add(calc_pos(x, y)) = true;
        }

        let mut dsu = const {
            let mut dsu = [0; N * N];

            let mut i = 0;
            while i < N * N {
                dsu[i] = i;
                i += 1;
            }
            dsu
        };
        let dsu = dsu.as_mut_ptr();

        unsafe fn get_set(i: usize, dsu: *mut usize) -> usize {
            if *dsu.add(i) == i {
                return i;
            }

            get_set(*dsu.add(i), dsu)
        }

        unsafe fn merge(a: usize, b: usize, dsu: *mut usize) {
            let a = get_set(a, dsu);
            let b = get_set(b, dsu);
            *dsu.add(b) = a;
        }

        for i in calc_pos(1, 0)..N * N {
            if *grid.add(i) {
                continue;
            }
            if !*grid.add(i - 1) {
                merge(i - 1, i, dsu);
            }
            if !*grid.add(i - N) {
                merge(i - N, i, dsu);
            }
        }

        input = input.sub(2);
        while input > start {
            let mut sl = 3;

            let mut y = *input as usize - b'0' as usize;
            let c = *input.sub(1) as usize;
            input = input.sub(2);
            if c >= b'0' as usize {
                y += (c - b'0' as usize) * 10;
                input = input.sub(1);
                sl += 1;
            }

            let mut x = *input as usize - b'0' as usize;
            let c = *input.sub(1) as usize;
            input = input.sub(2);
            if c >= b'0' as usize {
                x += (c - b'0' as usize) * 10;
                input = input.sub(1);
                sl += 1;
            }

            if !*grid.add(calc_pos(x - 1, y)) {
                merge(calc_pos(x - 1, y), calc_pos(x, y), dsu);
            }
            if !*grid.add(calc_pos(x + 1, y)) {
                merge(calc_pos(x + 1, y), calc_pos(x, y), dsu);
            }
            if !*grid.add(calc_pos(x, y - 1)) {
                merge(calc_pos(x, y - 1), calc_pos(x, y), dsu);
            }
            if !*grid.add(calc_pos(x, y + 1)) {
                merge(calc_pos(x, y + 1), calc_pos(x, y), dsu);
            }

            if get_set(calc_pos(0, 0), dsu) == get_set(calc_pos(70, 70), dsu) {
                return str::from_raw_parts(input.add(2), sl);
            }

            *grid.add(calc_pos(x, y)) = false;
        }

        unreachable_unchecked()
    }
}
