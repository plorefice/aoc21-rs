use std::{cmp::Ordering, str};

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &[&str]) -> usize {
    let mcbs: Vec<_> = (0..input[0].len())
        .map(|pos| b'0' + mcb(input, pos).unwrap() as u8)
        .collect();

    let gamma = usize::from_str_radix(str::from_utf8(&mcbs).unwrap(), 2).unwrap();
    let epsilon = (!gamma) & ((1 << mcbs.len()) - 1);

    gamma * epsilon
}

pub fn part_2(input: &[&str]) -> usize {
    let oxygen = scrub(input.to_vec(), false);
    let co2 = scrub(input.to_vec(), true);
    oxygen * co2
}

fn scrub(mut input: Vec<&str>, flip: bool) -> usize {
    let flipper = if flip { 1 } else { 0 }; // 🐬

    let mut pos = 0;
    while input.len() > 1 {
        let rating = b'0' + (mcb(&input, pos).unwrap_or(1) ^ flipper) as u8;
        input.retain(|l| l.as_bytes()[pos] == rating);
        pos += 1;
    }

    usize::from_str_radix(input[0], 2).unwrap()
}

fn mcb(input: &[&str], pos: usize) -> Option<usize> {
    let n = input.len();

    let zeros = input
        .iter()
        .fold(0, |z, l| if l.as_bytes()[pos] == b'0' { z + 1 } else { z });

    match zeros.cmp(&(n / 2)) {
        Ordering::Less => Some(1),
        Ordering::Equal => None,
        Ordering::Greater => Some(0),
    }
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day03.txt"))),
        3242606
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day03.txt"))),
        4856080
    }
}
