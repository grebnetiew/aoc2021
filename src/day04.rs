use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

pub type Board = Vec<Vec<BingoNumber>>;

#[derive(Debug, Clone, Copy)]
pub struct BingoNumber {
    val: u32,
    called: bool,
}

impl FromStr for BingoNumber {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BingoNumber {
            val: s.parse()?,
            called: false,
        })
    }
}

#[aoc_generator(day4)]
pub fn bingo_parse(input: &str) -> Result<(Vec<u32>, Vec<Board>), Box<dyn Error>> {
    let mut input = input.lines().filter(|l| !l.is_empty());

    let numbers = comma_separated_u32(input.next().ok_or_else(|| "not enough input".to_owned())?)?;

    let boards = input
        .chunks(5)
        .into_iter()
        .map(|ch| ch.map(bingo_line).collect())
        .collect::<Result<Vec<Board>, std::num::ParseIntError>>()?;

    Ok((numbers, boards))
}

fn comma_separated_u32(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input.split(',').map(str::parse).collect()
}

fn bingo_line(input: &str) -> Result<Vec<BingoNumber>, std::num::ParseIntError> {
    input.split_whitespace().map(str::parse).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let mut boards = input.1.clone();
    for n in &input.0 {
        for b in &mut boards {
            call(b, *n);
            if has_bingo(b) {
                return score(b, *n);
            }
        }
    }

    0
}

fn call(board: &mut Board, n: u32) {
    for line in board {
        for mut bn in line {
            if bn.val == n {
                bn.called = true;
            }
        }
    }
}

fn has_bingo(board: &Board) -> bool {
    for i in 0..5 {
        let (mut row, mut col) = (true, true);
        for j in 0..5 {
            row = row && board[i][j].called;
            col = col && board[j][i].called;
        }
        if row || col {
            return true;
        }
    }
    false
}

fn score(board: &Board, n: u32) -> u32 {
    board
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|bn| match bn.called {
            true => None,
            false => Some(bn.val),
        })
        .sum::<u32>()
        * n
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let mut boards = input.1.clone();
    let mut won = vec![false; boards.len()];
    for n in &input.0 {
        for (i, b) in boards.iter_mut().enumerate() {
            call(b, *n);
            if has_bingo(b) {
                won[i] = true;
                if won.iter().filter(|&w| !w).count() == 0 {
                    return score(b, *n);
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn sample1() {
        assert_eq!(part1(&bingo_parse(TEST_INPUT).unwrap()), 4512);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&bingo_parse(TEST_INPUT).unwrap()), 1924);
    }
}
