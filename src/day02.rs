use std::error::Error;

/// Returns where the submarine ends up after following the instructions.
#[aoc(day2, part1)]
pub fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let (mut x, mut y) = (0, 0);

    for (instruction, amount) in input.lines().filter_map(|l| l.split_once(' ')) {
        let n: u32 = amount.parse()?;
        match instruction.chars().next() {
            Some('f') => x += n,
            Some('d') => y += n,
            Some('u') => y -= n,
            _ => return Err("invalid instruction".into()),
        }
    }

    Ok(x * y)
}

/// Returns where the submarine ends up after following the instructions,
/// but now, 'down' and 'up' affect the direction in which 'forward' moves.
#[aoc(day2, part2)]
pub fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let (mut x, mut y, mut aim) = (0, 0, 0);

    for (instruction, amount) in input.lines().filter_map(|l| l.split_once(' ')) {
        let n: i32 = amount.parse()?;
        match instruction.chars().next() {
            Some('f') => {
                x += n;
                y += aim * n;
            }
            Some('d') => aim += n,
            Some('u') => aim -= n,
            _ => return Err("invalid instruction".into()),
        }
    }

    Ok(x * y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 150);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 900);
    }
}
