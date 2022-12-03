pub fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|section| {
            section
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap_or(0)
}

pub fn part2(input: &str) -> u32 {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|section| {
            section
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    calories.select_nth_unstable_by(2, |a, b| b.cmp(a));
    calories[0] + calories[1] + calories[2]
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day01.txt").unwrap()),
        71300
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day01.txt").unwrap()),
        209691
    );
}
