use std::intrinsics;

pub fn part1(input: &str) -> i32 {
    unsafe {
        let mut res = 0;

        let end = input.len();
        let mut input = input.as_ptr();
        let end = input.add(end);
        while input < end {
            let ax = (*input.add(12) as i32 - b'0' as i32) * 10 + *input.add(12 + 1) as i32
                - b'0' as i32;
            let ay = (*input.add(18) as i32 - b'0' as i32) * 10 + *input.add(18 + 1) as i32
                - b'0' as i32;
            let bx = (*input.add(33) as i32 - b'0' as i32) * 10 + *input.add(33 + 1) as i32
                - b'0' as i32;
            let by = (*input.add(39) as i32 - b'0' as i32) * 10 + *input.add(39 + 1) as i32
                - b'0' as i32;

            input = input.add(51);
            let mut x = (*input as i32 - b'0' as i32) * 100
                + (*input.add(1) as i32 - b'0' as i32) * 10
                + *input.add(2) as i32
                - b'0' as i32;
            input = input.add(3);
            while *input != b',' {
                x = x * 10 + (*input - b'0') as i32;
                input = input.add(1);
            }

            input = input.add(7);
            let mut y = (*input.sub(3) as i32 - b'0' as i32) * 100
                + (*input.sub(2) as i32 - b'0' as i32) * 10
                + *input.sub(1) as i32
                - b'0' as i32;
            while *input != b'\n' {
                y = y * 10 + (*input - b'0') as i32;
                input = input.add(1);
            }

            let de = ax * by - ay * bx;
            let a = intrinsics::unchecked_div(x * by - y * bx, de);
            let b = intrinsics::unchecked_div(y * ax - x * ay, de);

            if a >= 0 && b >= 0 && a * ax + b * bx == x && a * ay + b * by == y {
                res += a * 3 + b;
            }

            input = input.add(2);
        }

        res
    }
}

pub fn part2(input: &str) -> i64 {
    unsafe {
        let mut res = 0;

        let end = input.len();
        let mut input = input.as_ptr();
        let end = input.add(end);
        while input < end {
            let ax = (*input.add(12) as i64 - b'0' as i64) * 10 + *input.add(12 + 1) as i64
                - b'0' as i64;
            let ay = (*input.add(18) as i64 - b'0' as i64) * 10 + *input.add(18 + 1) as i64
                - b'0' as i64;
            let bx = (*input.add(33) as i64 - b'0' as i64) * 10 + *input.add(33 + 1) as i64
                - b'0' as i64;
            let by = (*input.add(39) as i64 - b'0' as i64) * 10 + *input.add(39 + 1) as i64
                - b'0' as i64;

            input = input.add(51);
            let mut x = (*input as i64 - b'0' as i64) * 100
                + (*input.add(1) as i64 - b'0' as i64) * 10
                + *input.add(2) as i64
                - b'0' as i64;
            input = input.add(3);
            while *input != b',' {
                x = x * 10 + (*input - b'0') as i64;
                input = input.add(1);
            }

            input = input.add(7);
            let mut y = (*input.sub(3) as i64 - b'0' as i64) * 100
                + (*input.sub(2) as i64 - b'0' as i64) * 10
                + *input.sub(1) as i64
                - b'0' as i64;
            while *input != b'\n' {
                y = y * 10 + (*input - b'0') as i64;
                input = input.add(1);
            }

            x += 10000000000000;
            y += 10000000000000;

            let de = ax * by - ay * bx;
            let a = intrinsics::unchecked_div(x * by - y * bx, de);
            let b = intrinsics::unchecked_div(y * ax - x * ay, de);

            if a >= 0 && b >= 0 && a * ax + b * bx == x && a * ay + b * by == y {
                res += a * 3 + b;
            }

            input = input.add(2);
        }

        res
    }
}
