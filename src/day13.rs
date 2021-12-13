pub struct Fold {
    is_horizontal: bool,
    xy: usize,
}

pub struct Point {
    x: usize,
    y: usize,
}

#[aoc_generator(day13)]
pub fn parse_points_and_folds(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let points = input
        .lines()
        .filter_map(|l| l.split_once(','))
        .map(|(x, y)| Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
        .collect();
    let folds = input
        .lines()
        .filter_map(|l| l.split_once('='))
        .map(|(text, val)| Fold {
            is_horizontal: text.ends_with('y'),
            xy: val.parse().unwrap(),
        })
        .collect();
    (points, folds)
}

#[aoc(day13, part1)]
pub fn part1(input: &(Vec<Point>, Vec<Fold>)) -> usize {
    // How many dots are visible after completing just the first fold
    // instruction on your transparent paper?
    let grid = points_to_grid(&input.0);
    let fold = &input.1[0];
    let mut total = 0;
    if fold.is_horizontal {
        for y in 0..fold.xy {
            for x in 0..grid[0].len() {
                if grid[y][x] | grid[grid.len() - 1 - y][x] {
                    total += 1;
                }
            }
        }
    } else {
        for x in 0..fold.xy {
            for y in 0..grid.len() {
                if grid[y][x] | grid[y][grid[0].len() - 1 - x] {
                    total += 1;
                }
            }
        }
    }
    total
}

#[aoc(day13, part2)]
pub fn part2(input: &(Vec<Point>, Vec<Fold>)) -> String {
    let mut grid = points_to_grid(&input.0);
    let mut xmax = grid[0].len();
    let mut ymax = grid.len();
    for fold in &input.1 {
        if fold.is_horizontal {
            for y in 0..fold.xy {
                for x in 0..xmax {
                    grid[y][x] |= grid[ymax - 1 - y][x];
                }
            }
            ymax = fold.xy;
        } else {
            for x in 0..fold.xy {
                #[allow(clippy::needless_range_loop)]
                for y in 0..ymax {
                    grid[y][x] |= grid[y][xmax - 1 - x];
                }
            }
            xmax = fold.xy;
        }
    }
    display(&grid, xmax, ymax)
}

fn points_to_grid(points: &[Point]) -> Vec<Vec<bool>> {
    let xmax = points.iter().map(|p| p.x).max().unwrap() + 1;
    let ymax = points.iter().map(|p| p.y).max().unwrap() + 1;
    let mut grid = vec![vec![false; xmax]; ymax];
    for p in points {
        grid[p.y][p.x] = true;
    }
    grid
}

fn display(grid: &[Vec<bool>], xmax: usize, ymax: usize) -> String {
    grid.iter()
        .take(ymax)
        .map(|row| {
            row.iter()
                .take(xmax)
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
                + "\n"
        })
        .fold(String::from("\n"), |acc, x| acc + &x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_points_and_folds(TEST_INPUT)), 17);
    }
    #[test]
    fn sample2() {
        assert_eq!(
            part2(&parse_points_and_folds(TEST_INPUT)),
            "\n#####\n#...#\n#...#\n#...#\n#####\n.....\n.....\n"
        );
    }
}
