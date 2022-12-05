#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> ([Vec<char>; 9], Vec<(usize, usize, usize)>) {
    let (stacks_str, moves_str) = input[..input.len() - 1].split_once("\n\n").unwrap();
    let mut stacks = core::array::from_fn(|_| Vec::with_capacity(16));
    for line in stacks_str.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..9 {
            if line[4 * i + 1] != b' ' {
                stacks[i].push(line[4 * i + 1] as char);
            }
        }
    }
    let mut moves = Vec::with_capacity(512);
    for line in moves_str.lines() {
        let amount = line[5..line.len() - 12].parse().unwrap();
        let from = (line.as_bytes()[line.len() - 6] - b'1') as _;
        let to = (line.as_bytes()[line.len() - 1] - b'1') as _;
        moves.push((amount, from, to));
    }
    (stacks, moves)
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for (amount, from, to) in moves {
        for i in 1..=amount {
            stacks[to].push(stacks[from][stacks[from].len() - i]);
        }
        stacks[from].truncate(stacks[from].len() - amount);
    }
    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    for (amount, from, to) in moves {
        for i in (1..=amount).rev() {
            stacks[to].push(stacks[from][stacks[from].len() - i]);
        }
        stacks[from].truncate(stacks[from].len() - amount);
    }
    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day05.txt").unwrap()),
        "TQRFCBSJJ"
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day05.txt").unwrap()),
        "RMHFJNVFP"
    );
}
