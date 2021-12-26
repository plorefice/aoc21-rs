use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Chunk {
    n: i64,
    m: i64,
    div: i64,
}

pub fn parse_input(input: &str) -> Vec<Chunk> {
    input
        .lines()
        .chunks(18)
        .into_iter()
        .map(|chunk| {
            let instrs = chunk.collect_vec();

            let div = instrs[4]
                .split_ascii_whitespace()
                .nth(2)
                .unwrap()
                .parse()
                .unwrap();

            let n = instrs[5]
                .split_ascii_whitespace()
                .nth(2)
                .unwrap()
                .parse()
                .unwrap();

            let m = instrs[15]
                .split_ascii_whitespace()
                .nth(2)
                .unwrap()
                .parse()
                .unwrap();

            Chunk { n, m, div }
        })
        .collect()
}

pub fn part_1(chunks: Vec<Chunk>) -> i64 {
    solve(&mut HashSet::new(), &chunks, 13, 0, (1..=9).rev()).unwrap()
}

pub fn part_2(chunks: Vec<Chunk>) -> i64 {
    solve(&mut HashSet::new(), &chunks, 13, 0, 1..=9).unwrap()
}

fn solve<I>(
    states: &mut HashSet<(usize, i64, i64)>,
    chunks: &[Chunk],
    digit: usize,
    input: i64,
    range: I,
) -> Option<i64>
where
    I: Iterator<Item = i64> + Clone,
{
    for w in range.clone() {
        let output = compute_chunk(w, input, &chunks[13 - digit]);

        if digit == 0 {
            if output == 0 {
                return Some(w);
            } else {
                continue;
            }
        }

        // Prune duplicated states
        if !states.insert((digit, input, output)) {
            continue;
        }

        if let Some(monad) = solve(states, chunks, digit - 1, output, range.clone()) {
            return Some(monad + w * 10_i64.pow(digit as u32));
        }
    }

    None
}

// Manually JIT'd from input program
fn compute_chunk(w: i64, z: i64, chunk: &Chunk) -> i64 {
    let x = (z % 26) + chunk.n;
    let z = z / chunk.div;

    if x != w {
        z * 26 + w + chunk.m
    } else {
        z
    }
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day24.txt"))),
        79197919993985
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day24.txt"))),
        13191913571211
    }
}
