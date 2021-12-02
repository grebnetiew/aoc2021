use std::error::Error;

pub enum SubmarineInstruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[aoc_generator(day2)]
pub fn submarine_instructions(input: &str) -> Result<Vec<SubmarineInstruction>, Box<dyn Error>> {
    input
        .lines()
        .map(|s| match s.split_once(' ') {
            Some(("forward", n)) => Ok(SubmarineInstruction::Forward(n.parse()?)),
            Some(("down", n)) => Ok(SubmarineInstruction::Down(n.parse()?)),
            Some(("up", n)) => Ok(SubmarineInstruction::Up(n.parse()?)),
            _ => Err("invalid instruction".into()),
        })
        .collect()
}

/// Returns where the submarine ends up after following the instructions.
#[aoc(day2, part1)]
pub fn part1(input: &[SubmarineInstruction]) -> u32 {
    let (mut x, mut y) = (0, 0);
    for ins in input {
        match ins {
            SubmarineInstruction::Forward(n) => x += n,
            SubmarineInstruction::Down(n) => y += n,
            SubmarineInstruction::Up(n) => y -= n,
        }
    }
    x * y
}

/// Returns where the submarine ends up after following the instructions,
/// but now, 'down' and 'up' affect the direction in which 'forward' moves.
#[aoc(day2, part2)]
pub fn part2(input: &[SubmarineInstruction]) -> i32 {
    let (mut x, mut y, mut aim) = (0i32, 0i32, 0i32);
    for ins in input {
        match ins {
            SubmarineInstruction::Forward(n) => {
                x += *n as i32;
                y += aim * *n as i32;
            }
            SubmarineInstruction::Down(n) => aim += *n as i32,
            SubmarineInstruction::Up(n) => aim -= *n as i32,
        }
    }
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&submarine_instructions(TEST_INPUT).unwrap()), 150);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&submarine_instructions(TEST_INPUT).unwrap()), 900);
    }
}
