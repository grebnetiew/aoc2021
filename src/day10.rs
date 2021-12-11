#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(corrupted_delimiter)
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn corrupted_delimiter(line: &str) -> Option<char> {
    let mut buffer = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => buffer.push(c),
            ')' => {
                if buffer.pop() != Some('(') {
                    return Some(c);
                }
            }
            ']' => {
                if buffer.pop() != Some('[') {
                    return Some(c);
                }
            }
            '}' => {
                if buffer.pop() != Some('{') {
                    return Some(c);
                }
            }
            '>' => {
                if buffer.pop() != Some('<') {
                    return Some(c);
                }
            }
            _ => (),
        }
    }
    None
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let mut scores: Vec<u64> = input
        .lines()
        .filter(|line| corrupted_delimiter(line).is_none())
        .map(open_delimiters)
        .map(|delims| {
            delims.iter().rfold(0, |acc, x| {
                acc * 5
                    + match x {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();
    let n = scores.len() / 2;
    let (_, res, _) = scores.select_nth_unstable(n);
    *res
}

fn open_delimiters(line: &str) -> Vec<char> {
    let mut buffer = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => buffer.push(c),
            ')' | ']' | '}' | '>' => {
                buffer.pop();
            }
            _ => (),
        };
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn sample1() {
        assert_eq!(part1(TEST_INPUT), 26397);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST_INPUT), 288957);
    }
}
