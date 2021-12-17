use crate::bits::{parse_hex_to_bin, Packet};

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<u8> {
    parse_hex_to_bin(input)
}

#[aoc(day16, part1)]
pub fn part1(input: &[u8]) -> u32 {
    let (pack, _) = Packet::parse(input);
    pack.version_sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &[u8]) -> u64 {
    let (pack, _) = Packet::parse(input);
    pack.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_hex_to_bin("8A004A801A8002F478")), 16);
        assert_eq!(part1(&parse_hex_to_bin("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&parse_hex_to_bin("C0015000016115A2E0802F182340")), 23);
        assert_eq!(
            part1(&parse_hex_to_bin("A0016C880162017C3686B18A3D4780")),
            31
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_hex_to_bin("C200B40A82")), 3);
        assert_eq!(part2(&parse_hex_to_bin("04005AC33890")), 54);
        assert_eq!(part2(&parse_hex_to_bin("880086C3E88112")), 7);
        assert_eq!(part2(&parse_hex_to_bin("CE00C43D881120")), 9);
        assert_eq!(part2(&parse_hex_to_bin("D8005AC2A8F0")), 1);
        assert_eq!(part2(&parse_hex_to_bin("F600BC2D8F")), 0);
        assert_eq!(part2(&parse_hex_to_bin("9C005AC2F8F0")), 0);
        assert_eq!(part2(&parse_hex_to_bin("9C0141080250320F1802104A08")), 1);
    }
}
