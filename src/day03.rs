fn parse_rucksack(rucksack: &[u8]) -> u64 {
    rucksack.iter().fold(0, |n, c| {
        let c = if *c <= b'Z' { c - b'A' + 26 } else { c - b'a' };
        n | 1 << c
    })
}

pub fn part1(input: &str) -> u32 {
    input.as_bytes()[..input.len() - 1]
        .split(|c| *c == b'\n')
        .map(|line| {
            let common =
                parse_rucksack(&line[..line.len() / 2]) & parse_rucksack(&line[line.len() / 2..]);
            let mut priority = 0;
            for p in 0..52 {
                if common & (1 << p) != 0 {
                    priority += p + 1;
                }
            }
            priority
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut rucksacks = input.as_bytes()[..input.len() - 1]
        .split(|c| *c == b'\n')
        .map(parse_rucksack);
    let mut priority = 0;
    // core::iter::Iterator::array_chunks is unstable
    while let Some(r1) = rucksacks.next() {
        let r2 = rucksacks.next().unwrap();
        let r3 = rucksacks.next().unwrap();
        let common = r1 & r2 & r3;
        for p in 0..52 {
            if common & (1 << p) != 0 {
                priority += p + 1;
            }
        }
    }
    priority
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day03.txt").unwrap()),
        7428
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day03.txt").unwrap()),
        2650
    );
}
