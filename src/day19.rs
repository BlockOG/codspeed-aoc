use core::str;

#[target_feature(enable = "popcnt")]
const unsafe fn phf(i: u8) -> usize {
    i.count_ones() as usize - (i & 0b10) as usize - 1
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
unsafe fn part1_inner(input: &str) -> u32 {
    let end = input.len();
    let mut input = input.as_ptr();
    let end = input.add(end);

    let mut trie = [(false, 0); 1 + 5 * 2000];
    let trie = trie.as_mut_ptr();
    let mut latest = 1;

    loop {
        let mut curr = trie;
        while *input >= b'a' {
            if (*curr).1 == 0 {
                (*curr).1 = latest;
                latest += 5;
            }

            curr = trie.add((*curr).1).add(phf(*input));
            input = input.add(1);
        }

        (*curr).0 = true;

        input = input.add(2);
        if *input.sub(2) == b'\n' {
            break;
        }
    }

    let mut res = 0;

    while input != end {
        let mut dp = 1u64;

        let start_curr = input;
        while *input != b'\n' {
            let mut curr_inp = input;
            let mut curr = trie;
            while *curr_inp != b'\n' && (*curr).1 != 0 {
                if (*curr).0 && (dp & 1 << input.offset_from(start_curr)) != 0 {
                    dp |= 1 << curr_inp.offset_from(start_curr);
                }

                curr = trie.add((*curr).1).add(phf(*curr_inp));
                curr_inp = curr_inp.add(1);
            }

            if (*curr).0 && (dp & 1 << input.offset_from(start_curr)) != 0 {
                dp |= 1 << curr_inp.offset_from(start_curr);
            }

            input = input.add(1);
        }

        res += ((dp & 1 << input.offset_from(start_curr)) != 0) as u32;
        input = input.add(1);
    }

    res
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
unsafe fn part2_inner(input: &str) -> u64 {
    let end = input.len();
    let mut input = input.as_ptr();
    let end = input.add(end);

    let mut trie = [(false, 0); 1 + 5 * 2000];
    let trie = trie.as_mut_ptr();
    let mut latest = 1;

    loop {
        let mut curr = trie;
        while *input >= b'a' {
            if (*curr).1 == 0 {
                (*curr).1 = latest;
                latest += 5;
            }

            curr = trie.add((*curr).1).add(phf(*input));
            input = input.add(1);
        }

        (*curr).0 = true;

        input = input.add(2);
        if *input.sub(2) == b'\n' {
            break;
        }
    }

    let mut res = 0;

    while input != end {
        let mut dp = const {
            let mut dp = [0u64; 61];
            dp[0] = 1;
            dp
        };
        let dp = dp.as_mut_ptr();

        let start_curr = input;
        while *input != b'\n' {
            let mut curr_inp = input;
            let mut curr = trie;
            while *curr_inp != b'\n' && (*curr).1 != 0 {
                if (*curr).0 {
                    *dp.offset(curr_inp.offset_from(start_curr)) +=
                        *dp.offset(input.offset_from(start_curr));
                }

                curr = trie.add((*curr).1).add(phf(*curr_inp));
                curr_inp = curr_inp.add(1);
            }

            if (*curr).0 {
                *dp.offset(curr_inp.offset_from(start_curr)) +=
                    *dp.offset(input.offset_from(start_curr));
            }

            input = input.add(1);
        }

        res += *dp.offset(input.offset_from(start_curr));
        input = input.add(1);
    }

    res
}

pub fn part1(input: &str) -> u32 {
    unsafe { part1_inner(input) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { part2_inner(input) }
}
