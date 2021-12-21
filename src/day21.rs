use std::{collections::HashMap, mem::swap};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Universe {
    positions: (u32, u32),
    scores: (u32, u32),
}

pub fn parse_input(input: &str) -> (u32, u32) {
    input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn part_1(input: (u32, u32)) -> u32 {
    deterministic_game(input)
}

pub fn part_2(positions: (u32, u32)) -> u64 {
    let score = dirac_game(
        &mut HashMap::new(),
        Universe {
            positions,
            scores: (0, 0),
        },
    );

    score.0.max(score.1)
}

fn deterministic_game((mut p1, mut p2): (u32, u32)) -> u32 {
    let mut scores = (0, 0);
    let mut rolls = 0;
    let mut die = 0;

    while scores.1 < 1000 {
        for _ in 0..3 {
            p1 += die + 1;
            die = (die + 1) % 100;
        }
        rolls += 3;

        while p1 > 10 {
            p1 -= 10;
        }
        scores.0 += p1;

        swap(&mut p1, &mut p2);
        swap(&mut scores.0, &mut scores.1);
    }

    scores.0 * rolls
}

fn dirac_game(cache: &mut HashMap<Universe, (u64, u64)>, universe: Universe) -> (u64, u64) {
    let Universe { positions, scores } = universe;

    // Exit condition
    if scores.1 >= 21 {
        return (0, 1);
    }

    // If we have already visited this universe, we already know the tally
    if let Some(&wins) = cache.get(&universe) {
        return wins;
    }

    let mut wins = (0, 0);

    // Iterate over all possible sums resulting from throwing 3d3s
    for roll in [
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ] {
        let mut pos = positions.0 + roll;
        if pos > 10 {
            pos -= 10;
        }

        // By swapping positions we can iterate on P1 only, saving us some headaches
        let new_wins = dirac_game(
            cache,
            Universe {
                positions: (positions.1, pos),
                scores: (scores.1, scores.0 + pos),
            },
        );

        wins.0 += new_wins.1;
        wins.1 += new_wins.0;
    }

    cache.insert(universe, wins);

    wins
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day21.txt"))),
        432450
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day21.txt"))),
        138508043837521
    }
}
