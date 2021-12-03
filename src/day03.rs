#[aoc(day3, part1)]
pub fn part1(input: &str) -> Result<u32, String> {
    let sums: Vec<u32> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .reduce(|v, w| v.iter().zip(w).map(|(x, y)| x + y).collect())
        .ok_or("not enough inputs")?;
    let n = input.lines().count();

    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..sums.len() {
        if sums[sums.len() - i - 1] >= (n / 2) as u32 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    Ok(gamma * epsilon)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> Result<u32, String> {
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| if c == '1' { 1 } else { 0 }).collect())
        .collect();

    // To find oxygen generator rating, determine the most common value (0 or 1) in the
    // current bit position, and keep only numbers with that bit in that position. If 0
    // and 1 are equally common, keep values with a 1 in the position being considered.
    let oxy = determine_rating(&input, true)?;

    // To find CO2 scrubber rating, determine the least common value (0 or 1) in the
    // current bit position, and keep only numbers with that bit in that position. If 0
    // and 1 are equally common, keep values with a 0 in the position being considered.
    let co2 = determine_rating(&input, false)?;

    Ok(oxy * co2)
}

fn determine_rating(vec: &[Vec<u8>], oxy: bool) -> Result<u32, String> {
    let mut filtered = vec.to_owned();
    let mut position = 0;
    while filtered.len() > 1 {
        let most_common_at = match oxy {
            true => most_common(&filtered, position),
            false => 1 - most_common(&filtered, position),
        };
        filtered = filter_position_value(&filtered, position, most_common_at);
        position += 1;
    }
    Ok(bin_to_dec(
        filtered.get(0).ok_or("None left after filtering!")?,
    ))
}

/// Returns which value most often occurs on position `position`. If 0 and 1 are equally
/// common, returns 1.
fn most_common(vec: &[Vec<u8>], position: usize) -> u8 {
    let n = vec.len();
    let sum: usize = vec.iter().map(|v| v[position] as usize).sum();
    if sum * 2 < n {
        0
    } else {
        1
    }
}

fn filter_position_value(vec: &[Vec<u8>], position: usize, value: u8) -> Vec<Vec<u8>> {
    vec.iter()
        .cloned()
        .filter(|v| v[position] == value)
        .collect()
}

fn bin_to_dec(vec: &[u8]) -> u32 {
    vec.iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| (x as u32) << i)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn sample1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 198);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 230);
    }
}
