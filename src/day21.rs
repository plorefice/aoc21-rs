use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicDie {
    value: u32,
    rolls: u32,
}

impl DeterministicDie {
    pub fn roll(&mut self) -> u32 {
        let res = self.value + 1;
        self.value = res % 100;
        self.rolls += 1;
        res
    }
}

pub fn parse_input(input: &str) -> (u32, u32) {
    input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn part_1((mut p1, mut p2): (u32, u32)) -> u32 {
    let mut scores = (0, 0);
    let mut p1_turn = true;

    let mut die = DeterministicDie::default();

    while scores.0 < 1000 && scores.1 < 1000 {
        let roll = die.roll() + die.roll() + die.roll();

        let (pos, score) = if p1_turn {
            (&mut p1, &mut scores.0)
        } else {
            (&mut p2, &mut scores.1)
        };

        *pos += roll;
        while *pos > 10 {
            *pos -= 10;
        }
        *score += *pos;

        p1_turn = !p1_turn;
    }

    scores.0.min(scores.1) * die.rolls
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Universe {
    positions: (u32, u32),
    scores: (u32, u32),

    p1_turn: bool,
    rolls: [u8; 3],
}

pub fn part_2(positions: (u32, u32)) -> u64 {
    let origin = Universe {
        positions,
        scores: (0, 0),
        p1_turn: true,
        rolls: [0, 0, 0],
    };

    let mut visited = HashMap::new();

    let final_score = play(&mut visited, origin);

    final_score.0.max(final_score.1)
}

fn play(visited: &mut HashMap<Universe, (u64, u64)>, mut universe: Universe) -> (u64, u64) {
    // If we have already visited this universe, we already know the tally
    if let Some(&wins) = visited.get(&universe) {
        return wins;
    }

    // Resolve score if the die was rolled three times
    if universe.rolls.iter().all(|r| *r != 0) {
        let (pos, score) = if universe.p1_turn {
            (&mut universe.positions.0, &mut universe.scores.0)
        } else {
            (&mut universe.positions.1, &mut universe.scores.1)
        };

        *pos += universe.rolls.iter().sum::<u8>() as u32;
        while *pos > 10 {
            *pos -= 10;
        }
        *score += *pos;

        if universe.scores.0 >= 21 {
            return (1, 0);
        } else if universe.scores.1 >= 21 {
            return (0, 1);
        } else {
            // Reset universe die and change turn
            universe.rolls = [0, 0, 0];
            universe.p1_turn = !universe.p1_turn;
        }
    }

    // If we get here, no player won in this round, so we need to simulate more universes
    let mut wins = (0, 0);

    // TODO: probably could use a SmallVec and avoid this linear search
    let idx = universe.rolls.iter().find_position(|&&r| r == 0).unwrap().0;

    // Throw the dice and simulate the next 3 universes
    for roll in 1..=3 {
        let new_universe = Universe {
            rolls: {
                let mut rolls = universe.rolls;
                rolls[idx] = roll;
                rolls
            },
            ..universe
        };

        let new_wins = play(visited, new_universe);
        wins.0 += new_wins.0;
        wins.1 += new_wins.1;
    }

    visited.insert(universe, wins);

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
