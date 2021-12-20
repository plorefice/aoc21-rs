use std::collections::{hash_map::RandomState, HashSet};

use itertools::Itertools;

pub type Coord = [i32; 3];

pub fn parse_input(input: &str) -> Vec<Vec<Coord>> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .map(|l| {
                    let mut nums = l.split(',');
                    [
                        nums.next().unwrap().parse().unwrap(),
                        nums.next().unwrap().parse().unwrap(),
                        nums.next().unwrap().parse().unwrap(),
                    ]
                })
                .collect()
        })
        .collect()
}

pub fn part_1(input: Vec<Vec<Coord>>) -> usize {
    solve(input).0
}

pub fn part_2(input: Vec<Vec<Coord>>) -> i32 {
    let offsets = solve(input).1;

    offsets
        .iter()
        .combinations(2)
        .map(|os| {
            (os[0][0] - os[1][0]).abs() + (os[0][1] - os[1][1]).abs() + (os[0][2] - os[1][2]).abs()
        })
        .max()
        .unwrap()
}

fn solve(mut input: Vec<Vec<Coord>>) -> (usize, Vec<Coord>) {
    let mut probes: HashSet<Coord, RandomState> = HashSet::from_iter(input[0].iter().cloned());
    let mut offsets = vec![[0, 0, 0]; input.len()];
    let mut discovered = vec![0];

    while discovered.len() < input.len() {
        let mut found = None;

        'search: for &reference in discovered.iter().rev() {
            let scanner = input[reference].clone();

            for (i, sensor) in input.iter_mut().enumerate() {
                if discovered.contains(&i) {
                    continue;
                }

                for tran in TRANSFORMATIONS {
                    let transformed = transform(sensor, tran);

                    for origin in &scanner {
                        for pivot in &transformed {
                            let offset = [
                                origin[0] - pivot[0],
                                origin[1] - pivot[1],
                                origin[2] - pivot[2],
                            ];

                            let matching = transformed
                                .iter()
                                .filter(|elem| {
                                    scanner.contains(&[
                                        elem[0] + offset[0],
                                        elem[1] + offset[1],
                                        elem[2] + offset[2],
                                    ])
                                })
                                .count();

                            if matching >= 12 {
                                let offset_from_zero = [
                                    offset[0] + offsets[reference][0],
                                    offset[1] + offsets[reference][1],
                                    offset[2] + offsets[reference][2],
                                ];

                                for x in &transformed {
                                    probes.insert([
                                        x[0] + offset_from_zero[0],
                                        x[1] + offset_from_zero[1],
                                        x[2] + offset_from_zero[2],
                                    ]);
                                }

                                offsets[i] = offset_from_zero;
                                *sensor = transformed;
                                found = Some(i);

                                break 'search;
                            }
                        }
                    }
                }
            }
        }

        discovered.push(found.unwrap());
    }

    (probes.len(), offsets)
}

fn transform(input: &[Coord], transform: [i32; 6]) -> Vec<Coord> {
    let mut res = Vec::with_capacity(input.len());

    for coord in input {
        res.push([
            coord[transform[0] as usize] * transform[3],
            coord[transform[1] as usize] * transform[4],
            coord[transform[2] as usize] * transform[5],
        ]);
    }

    res
}

// ¯\_(ツ)_/¯
const TRANSFORMATIONS: [[i32; 6]; 24] = [
    [0, 1, 2, 1, 1, 1],
    [0, 2, 1, 1, 1, -1],
    [0, 1, 2, 1, -1, -1],
    [0, 2, 1, 1, -1, 1],
    [0, 1, 2, -1, 1, -1],
    [0, 2, 1, -1, 1, 1],
    [0, 1, 2, -1, -1, 1],
    [0, 2, 1, -1, -1, -1],
    [1, 0, 2, 1, 1, -1],
    [1, 2, 0, 1, -1, -1],
    [1, 0, 2, 1, -1, 1],
    [1, 2, 0, 1, 1, 1],
    [1, 0, 2, -1, 1, 1],
    [1, 2, 0, -1, 1, -1],
    [1, 0, 2, -1, -1, -1],
    [1, 2, 0, -1, -1, 1],
    [2, 1, 0, 1, 1, -1],
    [2, 0, 1, 1, -1, -1],
    [2, 1, 0, 1, -1, 1],
    [2, 0, 1, 1, 1, 1],
    [2, 1, 0, -1, 1, 1],
    [2, 0, 1, -1, 1, -1],
    [2, 1, 0, -1, -1, -1],
    [2, 0, 1, -1, -1, 1],
];

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day19.txt"))),
        396
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day19.txt"))),
        11828
    }
}
