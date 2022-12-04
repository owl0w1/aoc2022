pub fn part1(input: &str) -> u32 {
    input.as_bytes()[..input.len() - 1]
        .split(|c| *c == b'\n')
        .map(|round| {
            let abc = round[0] - b'A';
            let xyz = round[2] - b'X';
            xyz as u32
                + 1
                + match (abc, xyz) {
                    (0, 0) => 3,
                    (0, 1) => 6,
                    (0, 2) => 0,
                    (1, 0) => 0,
                    (1, 1) => 3,
                    (1, 2) => 6,
                    (2, 0) => 6,
                    (2, 1) => 0,
                    (2, 2) => 3,
                    _ => panic!("invalid input"),
                }
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input.as_bytes()[..input.len() - 1]
        .split(|c| *c == b'\n')
        .map(|round| {
            let abc = round[0] - b'A';
            let xyz = round[2] - b'X';
            3 * xyz as u32
                + match (abc, xyz) {
                    (0, 0) => 3,
                    (0, 1) => 1,
                    (0, 2) => 2,
                    (1, 0) => 1,
                    (1, 1) => 2,
                    (1, 2) => 3,
                    (2, 0) => 2,
                    (2, 1) => 3,
                    (2, 2) => 1,
                    _ => panic!("invalid input"),
                }
        })
        .sum()
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day02.txt").unwrap()),
        12794
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day02.txt").unwrap()),
        14979
    );
}
