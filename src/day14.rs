use std::collections::HashMap;

use itertools::Itertools;

pub type Pairs = HashMap<(u8, u8), u64>;
pub type Rules = HashMap<(u8, u8), u8>;
pub type Letters = HashMap<u8, u64>;

pub fn parse_input(input: &str) -> (Pairs, Letters, Rules) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let pairs = template
        .bytes()
        .tuple_windows()
        .fold(HashMap::new(), |mut map, (a, b)| {
            *map.entry((a, b)).or_default() += 1;
            map
        });

    let letters = template.bytes().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_default() += 1;
        map
    });

    let rules = rules
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(" -> ").unwrap();
            (
                (left.as_bytes()[0], left.as_bytes()[1]),
                right.as_bytes()[0],
            )
        })
        .collect();

    (pairs, letters, rules)
}

pub fn part_1(input: (Pairs, Letters, Rules)) -> u64 {
    solve(input, 10)
}

pub fn part_2(input: (Pairs, Letters, Rules)) -> u64 {
    solve(input, 40)
}

pub fn solve((mut pairs, mut letters, rules): (Pairs, Letters, Rules), n: usize) -> u64 {
    for _ in 0..n {
        let mut next = HashMap::new();

        for ((a, b), n) in pairs {
            let rule = rules[&(a, b)];

            *letters.entry(rule).or_default() += n;

            *next.entry((a, rule)).or_default() += n;
            *next.entry((rule, b)).or_default() += n;
        }

        pairs = next;
    }

    letters.values().max().unwrap() - letters.values().min().unwrap()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day14.txt"))),
        3143
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day14.txt"))),
        4110215602456
    }
}
