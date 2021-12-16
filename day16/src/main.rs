use std::io::BufRead;

fn main() {
    let input = input::read_file("puzzles/day16.txt")
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let mut bits = Bits::from_hex(input.trim_end());
    println!("solution 1: {}", solve1(&mut bits));

    let mut bits = Bits::from_hex(input.trim_end());
    println!("solution 2: {}", solve2(&mut bits));
}

fn solve1(bits: &mut Bits) -> usize {
    let mut sum = 0;

    let version = bits.read(3);
    sum += version as usize;
    if bits.read(3) != 4 {
        if bits.read(1) == 0 {
            let now = bits.bits_left();
            let sub_bits = bits.read_big(15);
            while bits.bits_left() > now - sub_bits {
                sum += solve1(bits);
            }
        } else {
            let sub_packets = bits.read_big(11);
            for _ in 0..sub_packets {
                sum += solve1(bits);
            }
        }
    } else {
        while bits.read(5) & (1 << 4) != 0 {}
    }

    sum
}

fn solve2(bits: &mut Bits) -> usize {
    let _ = bits.read(3);
    let type_id = bits.read(3);
    if type_id == 4 {
        return decode_literal(bits);
    }

    let mut values = Vec::new();

    if bits.read(1) == 0 {
        let sub_bits = bits.read_big(15);
        let now = bits.bits_left();
        while bits.bits_left() > now - sub_bits {
            values.push(solve2(bits));
        }
    } else {
        let sub_packets = bits.read_big(11);
        for _ in 0..sub_packets {
            values.push(solve2(bits));
        }
    }

    match type_id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => (values[0] > values[1]) as usize,
        6 => (values[0] < values[1]) as usize,
        7 => (values[0] == values[1]) as usize,
        _ => unreachable!(),
    }
}

fn decode_literal(bits: &mut Bits) -> usize {
    let mut value: usize = 0;
    loop {
        let group: u8 = bits.read(5);
        value <<= 4;
        value |= (group & 0xf) as usize;
        if group & (1 << 4) == 0 {
            break value;
        }
    }
}

struct Bits {
    inner: Vec<u8>,
    c_byte: usize,
    c_bit: u8,
}

impl Bits {
    pub fn from_hex(hex: &str) -> Self {
        let bytes = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..=i + 1], 16).unwrap())
            .collect();
        Bits {
            inner: bytes,
            c_byte: 0,
            c_bit: 7,
        }
    }
    pub fn bits_left(&self) -> usize {
        if self.c_byte >= self.inner.len() {
            return 0;
        }
        (self.inner.len() - self.c_byte - 1) * 8 + self.c_bit as usize + 1
    }
    pub fn read(&mut self, bits: u8) -> u8 {
        if bits > 8 {
            panic!()
        }
        let mut dest = 0;
        for b in (0..bits).rev() {
            dest |= ((self.inner[self.c_byte] & (1 << self.c_bit)) >> self.c_bit) << b;

            if self.c_bit == 0 {
                self.c_bit = 7;
                self.c_byte += 1;
            } else {
                self.c_bit -= 1;
            }
        }
        dest
    }
    pub fn read_big(&mut self, bits: usize) -> usize {
        let mut dest = 0;
        for b in (0..bits).rev() {
            dest |= (((self.inner[self.c_byte] & (1 << self.c_bit)) >> self.c_bit) as usize) << b;

            if self.c_bit == 0 {
                self.c_bit = 7;
                self.c_byte += 1;
            } else {
                self.c_bit -= 1;
            }
        }
        dest
    }
}

#[test]
fn test_bits() {
    let mut bits = Bits::from_hex("D2FE28");
    assert_eq!(bits.inner, vec![0xD2, 0xFE, 0x28]);

    assert_eq!(bits.read(4), 13);
    assert_eq!(bits.read(4), 2);
    assert_eq!(bits.read(8), u8::MAX - 1);
    assert_eq!(bits.bits_left(), 8);
    assert_eq!(bits.read(8), 40);
    assert_eq!(bits.bits_left(), 0);
}

#[test]
fn test_decode_literal() {
    let mut bits = Bits::from_hex("D2FE28");
    let _ = bits.read(3);
    let _ = bits.read(3);
    assert_eq!(decode_literal(&mut bits), 2021);
}

#[test]
fn test_solve() {
    assert_eq!(
        solve1(&mut Bits::from_hex("C0015000016115A2E0802F182340")),
        23
    );

    assert_eq!(solve2(&mut Bits::from_hex("C200B40A82")), 3);
    assert_eq!(solve2(&mut Bits::from_hex("04005AC33890")), 54);
    assert_eq!(solve2(&mut Bits::from_hex("880086C3E88112")), 7);
    assert_eq!(solve2(&mut Bits::from_hex("CE00C43D881120")), 9);
    assert_eq!(solve2(&mut Bits::from_hex("D8005AC2A8F0")), 1);
    assert_eq!(solve2(&mut Bits::from_hex("F600BC2D8F")), 0);
    assert_eq!(solve2(&mut Bits::from_hex("9C005AC2F8F0")), 0);
    assert_eq!(solve2(&mut Bits::from_hex("9C0141080250320F1802104A08")), 1);
}
