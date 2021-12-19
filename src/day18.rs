type SnailToken = u8;

#[aoc_generator(day18)]
pub fn parse_snail_numbers(input: &str) -> Vec<Vec<SnailToken>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .filter_map(|ch| match ch {
                    b'[' | b']' => Some(ch),
                    b',' => None,
                    _ => Some(ch - b'0'),
                })
                .collect()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vec<SnailToken>]) -> u64 {
    let mut num = input[0].to_owned();
    for other in input.iter().skip(1) {
        num = add(&num, other);
        while reduce(&mut num) {}
    }
    magnitude(&num, 0).0
}

#[aoc(day18, part2)]
pub fn part2(input: &[Vec<SnailToken>]) -> u64 {
    let mut largest = 0;
    for i in input {
        for j in input {
            let mut num = add(i, j);
            while reduce(&mut num) {}
            largest = largest.max(magnitude(&num, 0).0);
        }
    }
    largest
}

fn add(x: &[SnailToken], y: &[SnailToken]) -> Vec<SnailToken> {
    let mut z = vec![b'['];
    z.extend_from_slice(x);
    z.extend_from_slice(y);
    z.push(b']');
    z
}

fn reduce(num: &mut Vec<SnailToken>) -> bool {
    let mut nesting = 0;
    for i in 0..num.len() {
        match num[i] {
            b'[' => {
                nesting += 1;
                if nesting > 4 {
                    reduce_nested(num, i);
                    return true;
                }
            }
            b']' => nesting -= 1,
            _ => (),
        }
    }
    for i in 0..num.len() {
        match num[i] {
            b'[' | b']' => (),
            _ => {
                if num[i] > 9 {
                    reduce_split(num, i);
                    return true;
                }
            }
        }
    }
    false
}

fn reduce_nested(num: &mut Vec<SnailToken>, i: usize) {
    // i is the opening bracket
    // find the first regular number to the left of i, and add num[i+1] to it
    for j in (0..i).rev() {
        if num[j] < 90 {
            num[j] += num[i + 1];
            break;
        }
    }
    // find the first regular number to the right of i+3, and add num[i+2] to it
    for j in i + 4..num.len() {
        if num[j] < 90 {
            num[j] += num[i + 2];
            break;
        }
    }
    // replace the current pair by 0
    num.splice(i..i + 4, [0]);
}

fn reduce_split(num: &mut Vec<SnailToken>, i: usize) {
    // i is the offending literal
    num.splice(i..i + 1, [b'[', num[i] / 2, (num[i] + 1) / 2, b']']);
}

fn magnitude(num: &[SnailToken], i: usize) -> (u64, usize) {
    match num[i] {
        b'[' => {
            let (left, consumed_left) = magnitude(num, i + 1);
            let (right, consumed_right) = magnitude(num, i + 1 + consumed_left);
            (3 * left + 2 * right, consumed_left + consumed_right + 2)
        }
        b']' => panic!("can not compute magnitude of ]"),
        _ => (num[i] as u64, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_snail_numbers("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(part1(&parse_snail_numbers(TEST_INPUT)), 3488);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_snail_numbers(TEST_INPUT)), 3993);
    }
}
