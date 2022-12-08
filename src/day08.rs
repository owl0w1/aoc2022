pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let col = input.iter().position(|c| *c == b'\n').unwrap();
    let row = input.len() / (col + 1);
    let mut visible = vec![false; row * col];
    for r in 0..row {
        let mut tallest = input[(row + 1) * r];
        visible[row * r] = true;
        for c in 0..col {
            let h = input[(row + 1) * r + c];
            if h > tallest {
                visible[row * r + c] = true;
                tallest = h;
            }
        }
        let mut tallest = input[(row + 1) * r + col - 1];
        visible[row * r + col - 1] = true;
        for c in (0..col).rev() {
            let h = input[(row + 1) * r + c];
            if h > tallest {
                visible[row * r + c] = true;
                tallest = h;
            }
        }
    }
    for c in 0..col {
        let mut tallest = input[c];
        visible[c] = true;
        for r in 0..row {
            let h = input[(row + 1) * r + c];
            if h > tallest {
                visible[row * r + c] = true;
                tallest = h;
            }
        }
        let mut tallest = input[(row + 1) * (row - 1) + c];
        visible[row * (row - 1) + c] = true;
        for r in (0..row).rev() {
            let h = input[(row + 1) * r + c];
            if h > tallest {
                visible[row * r + c] = true;
                tallest = h;
            }
        }
    }
    visible.into_iter().filter(|b| *b).count() as _
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let col = input.iter().position(|c| *c == b'\n').unwrap();
    let row = input.len() / (col + 1);
    let mut max_score = 0;
    for r in 0..row {
        for c in 0..col {
            let h = input[(row + 1) * r + c];
            let mut top = 0;
            for i in (0..r).rev() {
                top += 1;
                if input[(row + 1) * i + c] >= h {
                    break;
                }
            }
            let mut bottom = 0;
            for i in (r + 1)..row {
                bottom += 1;
                if input[(row + 1) * i + c] >= h {
                    break;
                }
            }
            let mut left = 0;
            for j in (0..c).rev() {
                left += 1;
                if input[(row + 1) * r + j] >= h {
                    break;
                }
            }
            let mut right = 0;
            for j in (c + 1)..col {
                right += 1;
                if input[(row + 1) * r + j] >= h {
                    break;
                }
            }
            let score = top * bottom * left * right;
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day08.txt").unwrap()),
        1835
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day08.txt").unwrap()),
        263670
    );
}
