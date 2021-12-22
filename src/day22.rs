use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

pub type Instruction = (bool, (Range<i32>, Range<i32>, Range<i32>));

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (command, coords) = l.split_once(' ').unwrap();
            let coords = coords
                .split(',')
                .map(|c| {
                    let (low, high) = c.split_once("..").unwrap();
                    low[2..].parse().unwrap()..high.parse().unwrap()
                })
                .collect_tuple()
                .unwrap();

            (command == "on", coords)
        })
        .collect()
}

pub fn part_1(instructions: Vec<Instruction>) -> usize {
    let mut points = HashSet::new();

    for (on, (xs, ys, zs)) in instructions.into_iter().take_while(|(_, (xs, ys, zs))| {
        xs.start >= -50
            && xs.end <= 50
            && ys.start >= -50
            && ys.end <= 50
            && zs.start >= -50
            && zs.end <= 50
    }) {
        for x in xs.start..=xs.end {
            for y in ys.start..=ys.end {
                for z in zs.start..=zs.end {
                    if on {
                        points.insert((x, y, z));
                    } else {
                        points.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    points.len()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day22.txt"))),
        570915
    }
}
