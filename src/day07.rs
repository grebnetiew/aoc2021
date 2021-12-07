#[inline]
#[aoc_generator(day7)]
fn comma_separated_u32(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    crate::day04::comma_separated_u32(input)
}

#[aoc(day7, part1)]
pub fn part1(input: &[u32]) -> u32 {
    (0..=*input.iter().max().unwrap())
        .map(|x| {
            input
                .iter()
                .map(|&y| if x > y { x - y } else { y - x })
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[u32]) -> u32 {
    (0..=*input.iter().max().unwrap())
        .map(|x| {
            input
                .iter()
                .map(|&y| if x > y { x - y } else { y - x })
                .map(|x| x * (x + 1) / 2)
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn sample1() {
        assert_eq!(part1(&comma_separated_u32(TEST_INPUT).unwrap()), 37);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&comma_separated_u32(TEST_INPUT).unwrap()), 168);
    }
}
