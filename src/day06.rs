#[aoc_generator(day6)]
fn histogram(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    let input = crate::day04::comma_separated_u32(input)?;
    let _last = input.iter().max().unwrap_or(&0) + 1;
    let mut hist = vec![0; 9];
    for i in input {
        hist[i as usize] += 1;
    }
    Ok(hist)
}

#[aoc(day6, part1)]
pub fn part1(input: &[u64]) -> u64 {
    let mut input = input.to_owned();
    lanternfish_simulate(&mut input, 80)
}

fn lanternfish_simulate(school: &mut Vec<u64>, steps: u64) -> u64 {
    for _ in 0..steps {
        lanternfish_step(school);
    }
    school.iter().sum()
}

fn lanternfish_step(school: &mut Vec<u64>) {
    let zeros = school[0];
    for i in 1..school.len() {
        school[i - 1] = school[i];
    }
    school[6] += zeros;
    school[8] = zeros;
}

#[aoc(day6, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let mut input = input.to_owned();
    lanternfish_simulate(&mut input, 256)
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&histogram(TEST_INPUT).unwrap()), 5934);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&histogram(TEST_INPUT).unwrap()), 26984457539);
    }
}
