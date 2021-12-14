use std::collections::HashMap;

pub fn parse_polymer(input: &str) -> (String, HashMap<(u8, u8), u8>) {
    let mut input = input.lines();
    let polymer = input.next().unwrap().to_string();
    let transforms = input
        .filter(|l| !l.is_empty())
        .map(|s| {
            let s = s.as_bytes();
            ((s[0], s[1]), *s.last().unwrap())
        })
        .collect();
    (polymer, transforms)
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let (mut polymer, transforms) = parse_polymer(input);
    for _ in 0..10 {
        polymer = transform_polymer(polymer, &transforms);
    }

    // count occurrence of individual letters
    let mut hist = HashMap::new();
    for ch in polymer.chars() {
        *hist.entry(ch).or_insert(0) += 1;
    }

    *hist.values().max().unwrap() - *hist.values().min().unwrap()
}

fn transform_polymer(polymer: String, transforms: &HashMap<(u8, u8), u8>) -> String {
    let mut new_polymer = String::new();
    new_polymer.push(polymer.chars().next().unwrap());

    polymer.as_bytes().windows(2).for_each(|w| {
        if let Some(b) = transforms.get(&(w[0], w[1])) {
            new_polymer.push(*b as char);
        }
        new_polymer.push(w[1] as char);
    });
    new_polymer
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let (polymer, transforms) = parse_polymer(input);
    let mut polymer_pairs = HashMap::new();
    for w in polymer.as_bytes().windows(2) {
        *polymer_pairs.entry((w[0], w[1])).or_insert(0) += 1;
    }

    for _ in 0..40 {
        polymer_pairs = transform_polymer_by_pairs(polymer_pairs, &transforms);
    }

    // count occurrence of individual letters, only the second of each pair
    // to avoid double counting
    let mut hist = HashMap::new();
    for (pair, freq) in polymer_pairs {
        *hist.entry(pair.1).or_insert(0) += freq;
    }

    // we still forgot the first letter
    *hist.entry(polymer.as_bytes()[0]).or_insert(0) += 1;

    *hist.values().max().unwrap() - *hist.values().min().unwrap()
}

fn transform_polymer_by_pairs(
    polymer: HashMap<(u8, u8), usize>,
    transforms: &HashMap<(u8, u8), u8>,
) -> HashMap<(u8, u8), usize> {
    let mut new_polymer = HashMap::new();
    for (pair, freq) in polymer {
        if let Some(ch) = transforms.get(&pair) {
            *new_polymer.entry((pair.0, *ch)).or_insert(0) += freq;
            *new_polymer.entry((*ch, pair.1)).or_insert(0) += freq;
        } else {
            new_polymer.insert(pair, freq);
        }
    }
    new_polymer
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn sample1() {
        assert_eq!(part1(TEST_INPUT), 1588);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(TEST_INPUT), 2188189693529);
    }
}
