#[aoc_generator(day11)]
pub fn parse_number_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as u32).collect())
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    let mut input = input.to_owned();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += octopus_step(&mut input);
    }
    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    let mut input = input.to_owned();
    let mut steps = 0;
    let total = input.len() * input[0].len();
    loop {
        steps += 1;
        if octopus_step(&mut input) == total {
            break steps;
        }
    }
}

fn octopus_step(grid: &mut [Vec<u32>]) -> usize {
    let (imax, jmax) = (grid.len(), grid[0].len());
    for i in 0..imax {
        for j in 0..jmax {
            increase(grid, (i, j));
        }
    }

    let mut flashes = 0;
    grid.iter_mut()
        .flat_map(|row| row.iter_mut())
        .filter(|x| **x > 9)
        .for_each(|x| {
            flashes += 1;
            *x = 0;
        });
    flashes
}

fn neighbours(p: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    if p.0 > 0 {
        if p.1 > 0 {
            res.push((p.0 - 1, p.1 - 1));
        }
        res.push((p.0 - 1, p.1));
        if p.1 < max.1 - 1 {
            res.push((p.0 - 1, p.1 + 1));
        }
    }

    if p.1 > 0 {
        res.push((p.0, p.1 - 1));
    }

    if p.1 < max.1 - 1 {
        res.push((p.0, p.1 + 1));
    }

    if p.0 < max.0 - 1 {
        if p.1 > 0 {
            res.push((p.0 + 1, p.1 - 1));
        }
        res.push((p.0 + 1, p.1));
        if p.1 < max.1 - 1 {
            res.push((p.0 + 1, p.1 + 1));
        }
    }
    res
}

fn increase(grid: &mut [Vec<u32>], p: (usize, usize)) {
    grid[p.0][p.1] += 1;
    if grid[p.0][p.1] == 10 {
        for q in neighbours(p, (grid.len(), grid[0].len())) {
            increase(grid, q);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_number_grid(TEST_INPUT)), 1656);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_number_grid(TEST_INPUT)), 195);
    }
}
