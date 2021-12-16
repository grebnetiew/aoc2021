#[derive(Debug, Clone)]
struct Packet {
    version: u32,
    op_type: u32,
    contents: PacketContents,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version as u32
            + match &self.contents {
                PacketContents::Literal(_) => 0,
                PacketContents::SubPackets(packets) => {
                    packets.iter().map(Packet::version_sum).sum::<u32>()
                }
            }
    }

    fn parse(input: &[u8]) -> (Self, usize) {
        let mut it = input.iter().copied();
        let version = bin_to_u64(&mut it, 3) as u32;
        let op_type = bin_to_u64(&mut it, 3) as u32;
        let (contents, sz) = match op_type {
            4 => PacketContents::parse_literal(&input[6..]),
            _ => PacketContents::parse_operator(&input[6..]),
        };
        (
            Self {
                version,
                op_type,
                contents,
            },
            sz + 6,
        )
    }

    fn value(&self) -> u64 {
        match &self.contents {
            PacketContents::Literal(v) => *v,
            PacketContents::SubPackets(packets) => match self.op_type {
                0 => packets.iter().map(Packet::value).sum(),
                1 => packets.iter().map(Packet::value).product(),
                2 => packets.iter().map(Packet::value).min().unwrap(),
                3 => packets.iter().map(Packet::value).max().unwrap(),
                5 => {
                    if packets[0].value() > packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].value() < packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].value() == packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("invalid operator type {}", self.op_type),
            },
        }
    }
}

#[derive(Debug, Clone)]
enum PacketContents {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

impl PacketContents {
    fn parse_literal(input: &[u8]) -> (Self, usize) {
        let mut input = input.iter().copied();
        let (mut value, mut read) = (0, 0);
        while let Some(cont) = input.next() {
            value = (value << 4) + bin_to_u64(&mut input, 4);
            read += 5;
            if cont == 0 {
                break;
            }
        }
        (PacketContents::Literal(value), read)
    }
    fn parse_operator(input: &[u8]) -> (Self, usize) {
        let mut it = input.iter().copied();
        let length_type = bin_to_u64(&mut it, 1);
        let mut packets = Vec::new();
        let mut cursor = 0;
        if length_type == 0 {
            let length = bin_to_u64(&mut it, 15) as usize;
            cursor += 16;
            while cursor < length + 16 {
                let (new_packet, parsed_length) = Packet::parse(&input[cursor..]);
                packets.push(new_packet);
                cursor += parsed_length;
            }
        } else {
            let amount = bin_to_u64(&mut it, 11) as usize;
            cursor += 12;
            while packets.len() < amount {
                let (new_packet, parsed_length) = Packet::parse(&input[cursor..]);
                packets.push(new_packet);
                cursor += parsed_length;
            }
        }
        (PacketContents::SubPackets(packets), cursor)
    }
}

#[aoc_generator(day16)]
pub fn parse_hex_to_bin(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|ch| u8::from_str_radix(&ch.to_string(), 16).expect("parse error"))
        .flat_map(|bits| (0..4).rev().map(move |i| (bits >> i) & 1))
        .collect()
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

fn bin_to_u64<T>(mut input: T, bits: usize) -> u64
where
    T: Iterator<Item = u8>,
{
    (0..bits)
        .filter_map(|_| input.next())
        .fold(0, |acc, x| acc << 1 | x as u64)
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
