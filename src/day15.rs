use pathfinding::directed::astar::astar;

type Point = (usize, usize);

#[aoc_generator(day15)]
pub fn parse_number_grid(input: &str) -> Vec<Vec<u32>> {
    crate::day11::parse_number_grid(input)
}

#[aoc(day15, part1)]
pub fn part1(input: &[Vec<u32>]) -> Option<u32> {
    let max = (input[0].len(), input.len());
    astar(
        &(0, 0),
        |&p| neighbours(input, p, max),
        |&p| heuristic(p, max),
        |&p| success(p, max),
    )
    .map(|(_path, cost)| cost)
}

#[aoc(day15, part2)]
pub fn part2(input: &[Vec<u32>]) -> Option<u32> {
    let max = (input[0].len(), input.len());
    let max_five = (input[0].len() * 5, input.len() * 5);
    astar(
        &(0, 0),
        |&p| neighbours_times_five(input, p, max),
        |&p| heuristic(p, max_five),
        |&p| success(p, max_five),
    )
    .map(|(_path, cost)| cost)
}

/// Returns the neighbours of `p` in the grid, along with the cost
/// of moving from `p` to each neighbour
fn neighbours(grid: &[Vec<u32>], p: Point, max: Point) -> Vec<(Point, u32)> {
    let mut res = Vec::new();

    if p.0 > 0 {
        res.push((p.0 - 1, p.1));
    }
    if p.1 > 0 {
        res.push((p.0, p.1 - 1));
    }
    if p.1 < max.1 - 1 {
        res.push((p.0, p.1 + 1));
    }
    if p.0 < max.0 - 1 {
        res.push((p.0 + 1, p.1));
    }

    res.into_iter().map(|p| (p, grid[p.1][p.0])).collect()
}

/// Returns the neighbours of `p` in the modified grid, along with
/// the cost of moving from `p` to each neighbour
fn neighbours_times_five(grid: &[Vec<u32>], p: Point, max: Point) -> Vec<(Point, u32)> {
    let mut res = Vec::new();

    if p.0 > 0 {
        res.push((p.0 - 1, p.1));
    }
    if p.1 > 0 {
        res.push((p.0, p.1 - 1));
    }
    if p.1 < max.1 * 5 - 1 {
        res.push((p.0, p.1 + 1));
    }
    if p.0 < max.0 * 5 - 1 {
        res.push((p.0 + 1, p.1));
    }

    res.into_iter()
        .map(|p| (p, cost_times_five(grid, p, max)))
        .collect()
}

/// Returns the value of `p` in the grid extended to 25 times its size
/// given the rules in the puzzle
fn cost_times_five(grid: &[Vec<u32>], p: Point, max: Point) -> u32 {
    let additional = p.0 / max.0 + p.1 / max.1;
    (grid[p.1 % max.1][p.0 % max.0] + additional as u32 - 1) % 9 + 1
}

/// Returns the manhattan distance between node `n` and the goal
fn heuristic(n: Point, max: Point) -> u32 {
    (max.0 - n.0 + max.1 - n.1 - 2) as u32
}

/// Returns whether `n` is the goal node
fn success(n: Point, max: Point) -> bool {
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
        assert_eq!(part1(&parse_number_grid(TEST_INPUT)), Some(40));
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_number_grid(TEST_INPUT)), Some(315));
    }
}
