use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub type Grid = HashMap<(i32, i32), u32>;

#[derive(Debug, Clone)]
pub struct Node {
    pos: (i32, i32),
    score: u32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

pub fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, n)| ((x as i32, y as i32), n.to_digit(10).unwrap()))
        })
        .collect()
}

pub fn part_1(grid: Grid) -> u32 {
    a_star(&grid, (0, 0), (99, 99))
}

pub fn part_2(mut grid: Grid) -> u32 {
    for (x, y) in (0..100).flat_map(|y| (0..100).map(move |x| (x, y))) {
        for (tx, ty) in (0..5).flat_map(|y| (0..5).map(move |x| (x, y))) {
            let mut v = grid[&(x, y)] + (tx + ty) as u32;
            if v > 9 {
                v -= 9;
            }
            grid.insert((x + tx * 100, y + ty * 100), v);
        }
    }

    a_star(&grid, (0, 0), (499, 499))
}

pub fn a_star(grid: &Grid, start: (i32, i32), end: (i32, i32)) -> u32 {
    let mut open = BinaryHeap::new();

    let mut g_scores: HashMap<_, _> = grid.keys().map(|&pos| (pos, std::u32::MAX)).collect();
    let mut f_scores: HashMap<_, _> = g_scores.clone();

    *g_scores.entry(start).or_default() = 0;
    *f_scores.entry(start).or_default() = estimate(grid, start, end);

    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    open.push(Reverse(Node {
        pos: start,
        score: 0,
    }));

    while let Some(Reverse(Node { pos, score })) = open.pop() {
        if pos == end {
            return score;
        }

        for (dx, dy) in deltas {
            let new_pos = (pos.0 + dx, pos.1 + dy);

            if let Some(&cost) = grid.get(&new_pos) {
                let tentative_score = g_scores[&pos] + cost;

                if tentative_score < g_scores[&new_pos] {
                    g_scores.insert(new_pos, tentative_score);
                    f_scores.insert(new_pos, tentative_score + estimate(grid, new_pos, end));

                    if !open.iter().any(|Reverse(n)| n.pos == new_pos) {
                        open.push(Reverse(Node {
                            pos: new_pos,
                            score: score + cost,
                        }));
                    }
                }
            }
        }
    }

    unreachable!()
}

fn estimate(grid: &Grid, a: (i32, i32), b: (i32, i32)) -> u32 {
    grid[&a] + ((b.0 - a.0).abs() + (b.1 - a.1).abs()) as u32
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day15.txt"))),
        626
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day15.txt"))),
        2966
    }
}
