use std::{num::ParseIntError, str::FromStr};

pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some(("forward", x)) => Ok(Command::Forward(x.parse()?)),
            Some(("down", x)) => Ok(Command::Down(x.parse()?)),
            Some(("up", x)) => Ok(Command::Up(x.parse()?)),
            _ => unreachable!(),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(str::parse::<Command>)
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn part_1(input: &[Command]) -> usize {
    let (pos, depth) = input.iter().fold((0, 0), |(pos, depth), cmd| match cmd {
        Command::Forward(x) => (pos + x, depth),
        Command::Down(x) => (pos, depth + x),
        Command::Up(x) => (pos, depth - x),
    });

    pos * depth
}

pub fn part_2(input: &[Command]) -> usize {
    let (pos, depth, _) = input
        .iter()
        .fold((0, 0, 0), |(pos, depth, aim), cmd| match cmd {
            Command::Forward(x) => (pos + x, depth + aim * x, aim),
            Command::Down(x) => (pos, depth, aim + x),
            Command::Up(x) => (pos, depth, aim - x),
        });

    pos * depth
}

crate::solutions!(
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day02.txt"))),
        1698735
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day02.txt"))),
        1594785890
    }
);
