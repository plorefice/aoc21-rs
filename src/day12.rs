use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split('-').collect_tuple().unwrap())
        .collect()
}

pub fn part_1(graph: &[(&str, &str)]) -> usize {
    solve(graph, false)
}

pub fn part_2(graph: &[(&str, &str)]) -> usize {
    solve(graph, true)
}

fn solve(graph: &[(&str, &str)], allow_dupe: bool) -> usize {
    let mut visiting = VecDeque::new();
    let mut visited = HashSet::new();
    let mut completed = 0;

    let destinations = precompute_destinations(graph);

    visiting.push_back(vec!["start"]);

    while let Some(path) = visiting.pop_front() {
        let last = path.last().unwrap();

        for &dest in &destinations[last] {
            // Early stop condition
            if dest == "end" {
                completed += 1;
                continue;
            }

            let is_dupe = dest.to_ascii_lowercase() == dest && path.contains(&dest);

            // Part 1: skip duplicated small caves
            if dest == "start" || (!allow_dupe && is_dupe) {
                continue;
            }

            // Part 2: allow up to one duplicated small cave
            if dest != "end" && allow_dupe && is_dupe && has_dupe(&path) {
                continue;
            }

            let mut path = path.clone();
            path.push(dest);

            if visited.insert(path.clone()) {
                visiting.push_back(path);
            }
        }
    }

    completed
}

fn precompute_destinations<'a>(graph: &'a [(&'a str, &'a str)]) -> HashMap<&'a str, Vec<&'a str>> {
    graph.iter().fold(HashMap::new(), |mut map, &(k, v)| {
        map.entry(k).or_default().push(v);
        map.entry(v).or_default().push(k);
        map
    })
}

fn has_dupe(path: &[&str]) -> bool {
    for &place in path.iter().filter(|&&s| s.to_ascii_lowercase() == s) {
        if path.iter().filter(|&&s| s == place).count() > 1 {
            return true;
        }
    }
    false
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day12.txt"))),
        4754
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day12.txt"))),
        143562
    }
}
