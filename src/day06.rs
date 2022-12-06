pub fn part1(input: &str) -> u32 {
    let input = &input.as_bytes()[..input.len() - 1];
    let mut s = 0;
    loop {
        if input[s] == input[s + 1] || input[s] == input[s + 2] || input[s] == input[s + 3] {
            s += 1;
            continue;
        }
        if input[s + 1] == input[s + 2] || input[s + 1] == input[s + 3] {
            s += 2;
            continue;
        }
        if input[s + 2] == input[s + 3] {
            s += 3;
            continue;
        }
        break;
    }
    (s + 4) as _
}

pub fn part2(input: &str) -> u32 {
    let input = &input.as_bytes()[..input.len() - 1];
    let mut s = 0;
    'outer: loop {
        let mut char_idxs = [usize::MAX; 26];
        for i in 0..14 {
            let c: usize = (input[s + i] - b'a') as _;
            if char_idxs[c] != usize::MAX {
                s += char_idxs[c] + 1;
                continue 'outer;
            }
            char_idxs[c] = i;
        }
        break;
    }
    (s + 14) as _
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day06.txt").unwrap()),
        1707
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day06.txt").unwrap()),
        3697
    );
}
