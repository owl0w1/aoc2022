fn parse_input(input: &str) -> impl Iterator<Item = Option<i32>> + '_ {
    input[..input.len() - 1].lines().map(|line| {
        if line.as_bytes()[0] == b'n' {
            None
        } else {
            Some(line[5..].parse().unwrap())
        }
    })
}

pub fn part1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut ans = 0;
    for instr in parse_input(input) {
        if cycle >= 220 {
            break;
        }
        if cycle % 40 == 20 {
            ans += cycle * x;
        }
        if let Some(d) = instr {
            cycle += 1;
            if cycle % 40 == 20 {
                ans += cycle * x;
            }
            x += d;
        }
        cycle += 1
    }
    ans
}

pub fn part2(input: &str) -> String {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut ans = String::with_capacity(245);
    for instr in parse_input(input) {
        if cycle >= 240 {
            break;
        }
        if cycle % 40 == 0 && cycle != 0 {
            ans.push('\n');
        }
        if x.abs_diff(cycle % 40) <= 1 {
            ans.push('#');
        } else {
            ans.push('.');
        }
        if let Some(d) = instr {
            cycle += 1;
            if cycle % 40 == 0 {
                ans.push('\n');
            }
            if x.abs_diff(cycle % 40) <= 1 {
                ans.push('#');
            } else {
                ans.push('.');
            }
            x += d;
        }
        cycle += 1
    }
    ans
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day10.txt").unwrap()),
        16880
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day10.txt").unwrap()),
        "###..#..#..##..####..##....##.###..###..\n\
         #..#.#.#..#..#....#.#..#....#.#..#.#..#.\n\
         #..#.##...#..#...#..#..#....#.###..#..#.\n\
         ###..#.#..####..#...####....#.#..#.###..\n\
         #.#..#.#..#..#.#....#..#.#..#.#..#.#.#..\n\
         #..#.#..#.#..#.####.#..#..##..###..#..#."
    );
}
