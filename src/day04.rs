pub fn part1(input: &str) -> u32 {
    // core::iter::Iterator::array_chunks is unstable
    input[..input.len() - 1]
        .lines()
        .map(|line| {
            let mut nums = line
                .split(|c: char| !c.is_ascii_digit())
                .map(|num| num.parse::<u32>().unwrap());
            let l1 = nums.next().unwrap();
            let r1 = nums.next().unwrap();
            let l2 = nums.next().unwrap();
            let r2 = nums.next().unwrap();
            (l1, r1, l2, r2)
        })
        .filter(|(l1, r1, l2, r2)| l1 <= l2 && r2 <= r1 || l2 <= l1 && r1 <= r2)
        .count() as _
}

pub fn part2(input: &str) -> u32 {
    // core::iter::Iterator::array_chunks is unstable
    input[..input.len() - 1]
        .lines()
        .map(|line| {
            let mut nums = line
                .split(|c: char| !c.is_ascii_digit())
                .map(|num| num.parse::<u32>().unwrap());
            let l1 = nums.next().unwrap();
            let r1 = nums.next().unwrap();
            let l2 = nums.next().unwrap();
            let r2 = nums.next().unwrap();
            (l1, r1, l2, r2)
        })
        .filter(|(l1, r1, l2, r2)| l1 <= r2 && l2 <= r1)
        .count() as _
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day04.txt").unwrap()),
        498
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day04.txt").unwrap()),
        859
    );
}
