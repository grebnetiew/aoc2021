#[aoc_generator(day6)]
fn histogram(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    let mut hist = vec![0; 9];
    for i in input.split(',').map(str::parse::<usize>) {
        // Panic if any number is above 8 - that's not legal in day06
        hist[i?] += 1;
    }
    Ok(hist)
}

fn lanternfish_simulate(school: &[u64], steps: u64) -> u64 {
    let mut school = school.to_owned();
    for _ in 0..steps {
        school.rotate_left(1);
        school[6] += school[8];
    }
    school.iter().sum()
}

#[aoc(day6, part1)]
pub fn part1(input: &[u64]) -> u64 {
    lanternfish_simulate(input, 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &[u64]) -> u64 {
    lanternfish_simulate(input, 256)
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
