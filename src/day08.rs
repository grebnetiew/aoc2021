use std::collections::HashMap;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| l.split_once(" | "))
        .map(|s| {
            s.1.split(' ')
                .map(str::len)
                .filter(|&x| x == 2 || x == 3 || x == 4 || x == 7)
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| l.split_once(" | "))
        .map(|(s1, s2)| {
            interpret_digits(
                s1.split(' ').map(str_to_bits).collect(),
                s2.split(' ').map(str_to_bits).collect(),
            )
        })
        .sum()
}

struct BidiMap(HashMap<u8, u32>);

impl BidiMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, k: u8, v: u32) {
        self.0.insert(k, v);
    }

    fn get(&self, k: u8) -> Option<&u32> {
        self.0.get(&k)
    }

    fn contains_key(&self, k: u8) -> bool {
        self.0.contains_key(&k)
    }

    fn get_reverse(&self, needle: u32) -> Option<u8> {
        self.0
            .iter()
            .find_map(|(&k, &v)| if v == needle { Some(k) } else { None })
    }

    fn must_get_reverse(&self, needle: u32) -> u8 {
        self.get_reverse(needle).unwrap()
    }
}

fn interpret_digits(samples: Vec<u8>, prompt: Vec<u8>) -> u32 {
    let mut answers = BidiMap::new();

    // Find 1 4 7 8
    for &s in &samples {
        match ones_count(s) {
            2 => answers.insert(s, 1),
            3 => answers.insert(s, 7),
            4 => answers.insert(s, 4),
            7 => answers.insert(s, 8),
            _ => (),
        };
    }

    // Find 9: contains 4, included in 8, len 6
    if let Some(s) = samples.iter().find(|&&s| {
        !answers.contains_key(s)
            && ones_count(s) == 6
            && subset(answers.must_get_reverse(8), s)
            && subset(s, answers.must_get_reverse(4))
    }) {
        answers.insert(*s, 9);
    }
    // Find 0: contains 7, included in 8, len 6
    if let Some(s) = samples.iter().find(|&&s| {
        !answers.contains_key(s)
            && ones_count(s) == 6
            && subset(answers.must_get_reverse(8), s)
            && subset(s, answers.must_get_reverse(7))
    }) {
        answers.insert(*s, 0);
    }
    // Find 6: only remaining of len 6
    if let Some(s) = samples
        .iter()
        .find(|&&s| !answers.contains_key(s) && ones_count(s) == 6)
    {
        answers.insert(*s, 6);
    }
    // Find 5: included in 6
    if let Some(s) = samples
        .iter()
        .find(|&&s| !answers.contains_key(s) && subset(answers.must_get_reverse(6), s))
    {
        answers.insert(*s, 5);
    }
    // Find 3: included in 9
    if let Some(s) = samples
        .iter()
        .find(|&&s| !answers.contains_key(s) && subset(answers.must_get_reverse(9), s))
    {
        answers.insert(*s, 3);
    }
    // Find 2: only one left
    if let Some(s) = samples.iter().find(|&&s| !answers.contains_key(s)) {
        answers.insert(*s, 2);
    }

    // Use answers to fill digits
    prompt
        .iter()
        .map(|s| answers.get(*s).expect("Answers incomplete"))
        .fold(0, |acc, x| acc * 10 + x)
}

fn subset(supers: u8, subs: u8) -> bool {
    // a is a subset of b if and only if (a | b) == b
    supers != subs && (supers | subs) == supers
}

fn str_to_bits(s: &str) -> u8 {
    s.bytes().map(|b| 1 << ((b - b'a') as usize)).sum()
}

fn ones_count(b: u8) -> usize {
    if b == 0 {
        0
    } else {
        (b & 1) as usize + ones_count(b >> 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn sample1() {
        assert_eq!(part1(TEST_INPUT), 26);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST_INPUT), 61229);
    }
}
