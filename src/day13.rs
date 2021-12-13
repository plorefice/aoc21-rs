use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fold {
    X(i32),
    Y(i32),
}

pub fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Fold>) {
    let (dots, folds) = input.split_once("\n\n").unwrap();

    let dots = dots
        .lines()
        .map(|l| {
            l.split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let folds = folds
        .lines()
        .map(|l| {
            let n = l[13..].parse().unwrap();

            if l.contains('x') {
                Fold::X(n)
            } else {
                Fold::Y(n)
            }
        })
        .collect();

    (dots, folds)
}

pub fn part_1((mut dots, folds): (HashSet<(i32, i32)>, Vec<Fold>)) -> usize {
    fold(&mut dots, folds[0]);
    dots.len()
}

pub fn part_2((mut dots, folds): (HashSet<(i32, i32)>, Vec<Fold>)) {
    for f in folds {
        fold(&mut dots, f);
    }

    let xmax = dots.iter().map(|(x, _)| *x).max().unwrap();
    let ymax = dots.iter().map(|(_, y)| *y).max().unwrap();

    for y in 0..=ymax {
        for x in 0..=xmax {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fold(dots: &mut HashSet<(i32, i32)>, fold: Fold) {
    *dots = dots
        .drain()
        .map(|(x, y)| match fold {
            Fold::X(n) => (n - (x - n).abs(), y),
            Fold::Y(n) => (x, n - (y - n).abs()),
        })
        .collect();
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day13.txt"))),
        706
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day13.txt"))),
        ()
    }
}
