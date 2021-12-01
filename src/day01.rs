#[aoc_generator(day1)]
pub fn one_u32_per_line(input: &str) -> Vec<u32> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

/// Returns how often two successive elements in the input increase.
#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

/// Returns how often two successive windows of three elements in the input
/// increase.
///
/// # Example
///
/// The sequence `[1, 1, 1, 2]` has two windows of three elements; the first
/// one sums to 3 whereas the second one sums to 4. The answer for this input
/// is therefore 1.
/// The answer for `[1, 1, 1, 0]` is 0.
#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> usize {
    input.windows(4).filter(|w| w[3] > w[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            part1(&one_u32_per_line(
                "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
            )),
            7
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            part2(&one_u32_per_line(
                "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
            )),
            5
        );
    }
}
