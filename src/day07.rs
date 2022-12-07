fn solve(input: &str) -> std::collections::HashMap<String, u32> {
    let mut dirs = std::collections::HashMap::with_capacity(256);
    let mut path = Vec::with_capacity(16);
    path.push("");
    for line in input.lines().skip(1) {
        if line == "$ ls" || &line[..3] == "dir" {
            continue;
        }
        if line == "$ cd .." {
            path.pop();
            continue;
        }
        if &line[..4] == "$ cd" {
            path.push(&line[5..]);
            continue;
        }
        let size: u32 = line.split_once(' ').unwrap().0.parse().unwrap();
        let mut ancestor_path = String::new();
        for dir in path.iter() {
            ancestor_path += dir;
            ancestor_path.push('/');
            dirs.entry(ancestor_path.clone())
                .and_modify(|s| *s += size)
                .or_insert(size);
        }
    }
    dirs
}

pub fn part1(input: &str) -> u32 {
    solve(input).into_values().filter(|s| *s < 100000).sum()
}

pub fn part2(input: &str) -> u32 {
    let dirs = solve(input);
    let target = dirs["/"] - 40000000;
    dirs.into_values().filter(|s| *s > target).min().unwrap()
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day07.txt").unwrap()),
        1844187
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day07.txt").unwrap()),
        4978279
    );
}
