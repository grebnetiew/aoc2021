#[aoc_generator(day6)]
#[inline]
fn comma_separated_u32(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    crate::day04::comma_separated_u32(input)
}

#[aoc(day6, part1)]
pub fn part1(input: &[u32]) -> usize {
    let mut input = input.to_owned();
    lanternfish_simulate(&mut input, 80)
}

fn lanternfish_simulate(school: &mut Vec<u32>, steps: usize) -> usize {
    for _ in 0..steps {
        lanternfish_step(school);
    }
    school.len()
}

fn lanternfish_step(school: &mut Vec<u32>) {
    let mut new_fish = 0;
    for fish in school.iter_mut() {
        if *fish == 0 {
            new_fish += 1;
            *fish = 6;
        } else {
            *fish -= 1;
        }
    }
    school.append(&mut vec![8; new_fish]);
}

#[aoc(day6, part2)]
pub fn part2(input: &[u32]) -> u64 {
    fast_sim(input, 18)
}

fn fast_sim(input: &[u32], n: usize) -> u64 {
    let mut time = 0;
    let mut n_fish = vec![0u64; 7];
    let initial: u64 = input.len();
    n_fish[0] = initial;

    while time + 7 < n {
        time += 7;
        for i in 0..7 {
            n_fish[(i + 2) % 7] += n_fish[i];
        }
        println!("> {} {:?}", time, n_fish);
    }
    let remaining = n - time;
    let mut residue = 0;
    for i in 0..remaining {
        // For the last few steps, we take one school, simulate it
    }

    n_fish.iter().sum() + residue
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&comma_separated_u32(TEST_INPUT).unwrap()), 5934);
    }
    #[test]
    fn sample2() {
        assert_eq!(
            part2(&comma_separated_u32(TEST_INPUT).unwrap()),
            26984457539
        );
    }
}
