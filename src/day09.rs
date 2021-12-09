use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn parse_number_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<u8>]) -> usize {
    find_lowest(input)
        .iter()
        .map(|&(x, y)| input[x][y] as usize + 1)
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<u8>]) -> Option<usize> {
    let mut lowest = find_lowest(input);

    let mut basin_sizes = Vec::new();

    while let Some(point) = lowest.pop() {
        // Flood fill
        let mut basin = HashSet::new();
        let mut queue = vec![point];

        while let Some(point) = queue.pop() {
            if input[point.0][point.1] == 9 {
                continue;
            }

            // remove from "lowest" - lowest points that are in the same basin shouldn't be treated twice
            lowest = lowest.into_iter().filter(|&p| p != point).collect();

            if basin.insert(point) {
                // was not yet present
                if point.0 != 0 {
                    queue.push((point.0 - 1, point.1));
                }
                if point.1 != 0 {
                    queue.push((point.0, point.1 - 1));
                }
                if point.0 != input.len() - 1 {
                    queue.push((point.0 + 1, point.1));
                }
                if point.1 != input[0].len() - 1 {
                    queue.push((point.0, point.1 + 1));
                }
            }
        }
        basin_sizes.push(basin.len());
    }

    basin_sizes.sort_unstable();
    let mut i = basin_sizes.iter();
    Some(i.next_back()? * i.next_back()? * i.next_back()?)
}

fn get_or(input: &[Vec<u8>], i: usize, j: usize, default: u8) -> u8 {
    *input.get(i).and_then(|v| v.get(j)).unwrap_or(&default)
}

fn find_lowest(input: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut lowest = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] < get_or(input, i.wrapping_sub(1), j, 9)
                && input[i][j] < get_or(input, i, j.wrapping_sub(1), 9)
                && input[i][j] < get_or(input, i + 1, j, 9)
                && input[i][j] < get_or(input, i, j + 1, 9)
            {
                lowest.push((i, j));
            }
        }
    }
    lowest
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_number_grid(TEST_INPUT)), 15);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_number_grid(TEST_INPUT)), Some(1134));
    }
}
