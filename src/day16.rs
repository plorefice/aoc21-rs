use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BitStream {
    rest: VecDeque<u8>,
    current: u8,
    remaining_in_current: usize,
}

impl BitStream {
    pub fn pop(&mut self, mut n: usize) -> u32 {
        let mut res = 0;

        while n > 0 {
            if self.remaining_in_current > 0 {
                res = (res << 1) | ((self.current >> 3) & 0x1) as u32;
                self.current <<= 1;
                self.remaining_in_current -= 1;
                n -= 1;
            } else {
                self.current = self.rest.pop_front().expect("out of bits");
                self.remaining_in_current = 4;
            }
        }

        res
    }

    pub fn remaining(&self) -> usize {
        self.rest.len() * 4 + self.remaining_in_current
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    version: u32,
    ptype: u32,
    payload: Payload,
}

impl Packet {
    pub fn parse(stream: &mut BitStream) -> Packet {
        let version = stream.pop(3);
        let ptype = stream.pop(3);
        let payload = Payload::parse(stream, ptype);

        Packet {
            version,
            ptype,
            payload,
        }
    }

    pub fn cumulative_version(&self) -> u32 {
        match self.payload {
            Payload::Literal(_) => self.version,
            Payload::Operator(ref pkts) => {
                self.version + pkts.iter().map(Packet::cumulative_version).sum::<u32>()
            }
        }
    }

    pub fn value(&self) -> u64 {
        match self.payload {
            Payload::Literal(n) => n,
            Payload::Operator(ref pkts) => match self.ptype {
                0 => pkts.iter().map(Packet::value).sum(),
                1 => pkts.iter().map(Packet::value).product(),
                2 => pkts.iter().map(Packet::value).min().unwrap(),
                3 => pkts.iter().map(Packet::value).max().unwrap(),
                5 => (pkts[0].value() > pkts[1].value()) as u64,
                6 => (pkts[0].value() < pkts[1].value()) as u64,
                7 => (pkts[0].value() == pkts[1].value()) as u64,
                _ => unreachable!("invalid packet type"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Payload {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl Payload {
    pub fn parse(stream: &mut BitStream, ptype: u32) -> Self {
        match ptype {
            4 => Self::parse_literal(stream),
            _ => Self::parse_operator(stream),
        }
    }

    fn parse_literal(stream: &mut BitStream) -> Self {
        let mut lit = 0;

        loop {
            let digit = stream.pop(5);

            lit = (lit << 4) | (digit & 0xf) as u64;

            if (digit & 0x10) == 0 {
                break;
            }
        }

        Self::Literal(lit)
    }

    fn parse_operator(stream: &mut BitStream) -> Self {
        let length_id = stream.pop(1);

        let pkts = if length_id == 0 {
            let to_be_read = stream.pop(15) as usize;
            let remaining_at_end = stream.remaining() - to_be_read;

            let mut pkts = Vec::new();
            while stream.remaining() != remaining_at_end {
                pkts.push(Packet::parse(stream));
            }

            pkts
        } else {
            (0..stream.pop(11)).map(|_| Packet::parse(stream)).collect()
        };

        Self::Operator(pkts)
    }
}

pub fn parse_input(input: &str) -> BitStream {
    BitStream {
        rest: input
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect(),
        current: 0,
        remaining_in_current: 0,
    }
}

pub fn part_1(mut stream: BitStream) -> u32 {
    Packet::parse(&mut stream).cumulative_version()
}

pub fn part_2(mut stream: BitStream) -> u64 {
    Packet::parse(&mut stream).value()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day16.txt"))),
        852
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day16.txt"))),
        19348959966392
    }
}
