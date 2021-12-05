use std::cmp::Ordering;

#[aoc_generator(day5)]
pub fn segments(input: &str) -> Result<Vec<Vec<usize>>, std::num::ParseIntError> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .flat_map(|point| point.split(',').map(str::parse))
                .collect()
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    let max_xy = bounds(input);
    let mut grid = vec![vec!(0; max_xy.0 + 1); max_xy.1 + 1];
    for line in input.iter().filter(|line| is_hor_vert(line)) {
        update_map_with_line(&mut grid, line);
    }

    count_multiply_crossed(&grid)
}

#[aoc(day5, part2)]
pub fn part2(input: &[Vec<usize>]) -> usize {
    let max_xy = bounds(input);
    let mut grid = vec![vec!(0; max_xy.0 + 1); max_xy.1 + 1];
    for line in input.iter() {
        update_map_with_line(&mut grid, line);
    }

    count_multiply_crossed(&grid)
}

fn bounds(input: &[Vec<usize>]) -> (usize, usize) {
    use std::cmp::max;
    input.iter().fold((0, 0), |acc, v| {
        (max(acc.0, max(v[0], v[2])), max(acc.1, max(v[1], v[3])))
    })
}

fn is_hor_vert(line: &[usize]) -> bool {
    line[0] == line[2] || line[1] == line[3]
}

fn update_map_with_line(grid: &mut Vec<Vec<usize>>, line: &[usize]) {
    let (mut x, mut y) = (line[0], line[1]);
    grid[y][x] += 1;
    while !(x == line[2] && y == line[3]) {
        match x.cmp(&line[2]) {
            Ordering::Greater => x -= 1,
            Ordering::Less => x += 1,
            Ordering::Equal => (),
        };
        match y.cmp(&line[3]) {
            Ordering::Greater => y -= 1,
            Ordering::Less => y += 1,
            Ordering::Equal => (),
        };
        grid[y][x] += 1;
    }
}

fn count_multiply_crossed(grid: &[Vec<usize>]) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&x| x > &1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&segments(TEST_INPUT).unwrap()), 5);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&segments(TEST_INPUT).unwrap()), 12);
    }
}
