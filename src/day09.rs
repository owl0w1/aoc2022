fn follow(follower: (i32, i32), followee: (i32, i32)) -> (i32, i32) {
    let (x1, y1) = follower;
    let (x2, y2) = followee;
    if x1.abs_diff(x2) > 1 || y1.abs_diff(y2) > 1 {
        (x1 + (x2 - x1).signum(), y1 + (y2 - y1).signum())
    } else {
        follower
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = ((i32, i32), u32)> + '_ {
    input[..input.len() - 1].lines().map(|line| {
        let dir = match line.as_bytes()[0] {
            b'L' => (-1, 0),
            b'R' => (1, 0),
            b'U' => (0, 1),
            b'D' => (0, -1),
            _ => panic!("invalid input"),
        };
        let step = line[2..].parse().unwrap();
        (dir, step)
    })
}

pub fn part1(input: &str) -> u32 {
    let mut tails = std::collections::HashSet::with_capacity(1 << 13);
    let mut head = (0, 0);
    let mut tail = (0, 0);
    tails.insert((0, 0));
    for ((dx, dy), step) in parse_input(input) {
        for _ in 0..step {
            head = (head.0 + dx, head.1 + dy);
            tail = follow(tail, head);
            tails.insert(tail);
        }
    }
    tails.len() as _
}

pub fn part2(input: &str) -> u32 {
    let mut tails = std::collections::HashSet::with_capacity(1 << 12);
    let mut rope = [(0, 0); 10];
    tails.insert((0, 0));
    for ((dx, dy), step) in parse_input(input) {
        for _ in 0..step {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..10 {
                let pos = follow(rope[i], rope[i - 1]);
                if pos == rope[i] {
                    break;
                }
                rope[i] = pos;
            }
            tails.insert(rope[9]);
        }
    }
    tails.len() as _
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day09.txt").unwrap()),
        6745
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day09.txt").unwrap()),
        2793
    );
}
