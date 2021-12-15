use pathfinding::directed::astar::astar;

#[aoc_generator(day15)]
pub fn parse_number_grid(input: &str) -> Vec<Vec<u32>> {
    crate::day11::parse_number_grid(input)
}

#[aoc(day15, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let max = (input[0].len(), input.len());
    let path = astar(
        &(0, 0),
        |&p| neighbours(input, p, max),
        |&p| heuristic(p, max),
        |&p| success(p, max),
    );
    path.unwrap().1
}

#[aoc(day15, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let max = (input[0].len(), input.len());
    let max_five = (input[0].len() * 5, input.len() * 5);
    let path = astar(
        &(0, 0),
        |&p| neighbours_times_five(input, p, max),
        |&p| heuristic(p, max_five),
        |&p| success(p, max_five),
    );
    path.unwrap().1
}

fn neighbours(
    grid: &[Vec<u32>],
    p: (usize, usize),
    max: (usize, usize),
) -> Vec<((usize, usize), u32)> {
    let mut res = Vec::new();
    if p.0 > 0 {
        res.push(((p.0 - 1, p.1), grid[p.1][p.0 - 1]));
    }

    if p.1 > 0 {
        res.push(((p.0, p.1 - 1), grid[p.1 - 1][p.0]));
    }

    if p.1 < max.1 - 1 {
        res.push(((p.0, p.1 + 1), grid[p.1 + 1][p.0]));
    }

    if p.0 < max.0 - 1 {
        res.push(((p.0 + 1, p.1), grid[p.1][p.0 + 1]));
    }
    res
}

fn neighbours_times_five(
    grid: &[Vec<u32>],
    p: (usize, usize),
    max: (usize, usize),
) -> Vec<((usize, usize), u32)> {
    let mut res = Vec::new();
    if p.0 > 0 {
        res.push(((p.0 - 1, p.1), grid_times_five(grid, (p.0 - 1, p.1), max)));
    }

    if p.1 > 0 {
        res.push(((p.0, p.1 - 1), grid_times_five(grid, (p.0, p.1 - 1), max)));
    }

    if p.1 < max.1 * 5 - 1 {
        res.push(((p.0, p.1 + 1), grid_times_five(grid, (p.0, p.1 + 1), max)));
    }

    if p.0 < max.0 * 5 - 1 {
        res.push(((p.0 + 1, p.1), grid_times_five(grid, (p.0 + 1, p.1), max)));
    }
    res
}

fn grid_times_five(grid: &[Vec<u32>], p: (usize, usize), max: (usize, usize)) -> u32 {
    let additional = p.0 / max.0 + p.1 / max.1;
    (grid[p.1 % max.1][p.0 % max.0] + additional as u32 - 1) % 9 + 1
}

fn heuristic(n: (usize, usize), max: (usize, usize)) -> u32 {
    (max.0 - n.0 + max.1 - n.1 - 2) as u32
}

fn success(n: (usize, usize), max: (usize, usize)) -> bool {
    max.0 == n.0 + 1 && max.1 == n.1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_number_grid(TEST_INPUT)), 40);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_number_grid(TEST_INPUT)), 315);
    }
}
