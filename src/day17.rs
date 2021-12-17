#[aoc_generator(day17)]
pub fn generator(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    let input = &input[15..];
    let (first, second) = input.split_once("..").unwrap();
    let (second, third) = second.split_once(',').unwrap();
    let third = &third[3..];
    let (third, fourth) = third.split_once("..").unwrap();
    vec![first, second, third, fourth]
        .into_iter()
        .map(str::parse)
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &[i32]) -> i32 {
    (0..input[2].abs())
        .filter_map(|yspeed| hit_target_with_yspeed(input[2], input[3], yspeed))
        .max()
        .expect("none of the shots hit the target at all")
}

#[aoc(day17, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut n = 0;
    for ys in input[2]..input[2].abs() {
        for xs in 0..input[1] + 1 {
            if hit_target(input, xs, ys) {
                n += 1;
            }
        }
    }
    n
}

// If can hit, returns the highest y
fn hit_target_with_yspeed(ymin: i32, ymax: i32, mut yspeed: i32) -> Option<i32> {
    let mut y = 0;
    let mut yrecord = 0;
    while y >= ymin {
        if y <= ymax {
            return Some(yrecord);
        }
        y += yspeed;
        yrecord = yrecord.max(y);
        yspeed -= 1;
    }
    None
}

fn hit_target(target: &[i32], mut xspeed: i32, mut yspeed: i32) -> bool {
    let (mut x, mut y) = (0, 0);
    while y >= target[2] && x <= target[1] {
        if y <= target[3] && x >= target[0] {
            return true;
        }
        x += xspeed;
        y += yspeed;
        xspeed -= xspeed.signum();
        yspeed -= 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn sample1() {
        assert_eq!(part1(&generator(TEST_INPUT).unwrap()), 45);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&generator(TEST_INPUT).unwrap()), 112);
    }
}
