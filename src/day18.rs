use std::{iter::Sum, ops::Add};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    data: Vec<(u64, i32)>,
}

impl Number {
    pub fn magnitude(&self) -> u64 {
        let mut stack = Vec::new();

        for &n in &self.data {
            stack.push(n);

            // Coalesce the top of the stack
            loop {
                match (stack.pop(), stack.pop()) {
                    (Some(last), None) => {
                        stack.push(last);
                        break;
                    }
                    (Some(r), Some(l)) => {
                        if r.1 == l.1 {
                            stack.push((3 * l.0 + 2 * r.0, r.1 - 1));
                        } else {
                            stack.push(l);
                            stack.push(r);
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        stack.pop().unwrap().0
    }

    fn reduce(&mut self) {
        // Keep going until self either can be exploded (first) or split (second)
        while self.explode().is_some() || self.split().is_some() {}
    }

    fn explode(&mut self) -> Option<()> {
        let (i, &(_, depth)) = self.data.iter().find_position(|(_, depth)| *depth >= 4)?;

        if i > 0 {
            self.data[i - 1].0 += self.data[i].0;
        }
        if i < self.data.len() - 2 {
            self.data[i + 2].0 += self.data[i + 1].0;
        }

        self.data.splice(i..i + 2, [(0, depth - 1)]);

        Some(())
    }

    fn split(&mut self) -> Option<()> {
        let (i, &(num, depth)) = self.data.iter().find_position(|(n, _)| *n >= 10)?;

        self.data
            .splice(i..i + 1, [(num / 2, depth + 1), ((num + 1) / 2, depth + 1)]);

        Some(())
    }
}

impl Add for Number {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (_, d) in &mut self.data {
            *d += 1;
        }
        for (n, d) in rhs.data {
            self.data.push((n, d + 1));
        }

        self.reduce();
        self
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut lhs = iter.next().unwrap();
        for rhs in iter {
            lhs = lhs + rhs;
        }
        lhs
    }
}

pub fn parse_input(input: &str) -> Vec<Number> {
    input
        .lines()
        .map(|line| {
            let mut data = Vec::new();
            let mut depth = -1;

            for c in line.chars() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => (),
                    _ => data.push((c.to_digit(10).unwrap() as u64, depth)),
                }
            }

            Number { data }
        })
        .collect()
}

pub fn part_1(numbers: Vec<Number>) -> u64 {
    numbers.into_iter().sum::<Number>().magnitude()
}

pub fn part_2(numbers: Vec<Number>) -> u64 {
    numbers
        .iter()
        .permutations(2)
        .map(|nums| (nums[0].clone() + nums[1].clone()).magnitude())
        .max()
        .unwrap()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day18.txt"))),
        4469
    },
    p2=> {
        part_2(parse_input(include_str!("../inputs/day18.txt"))),
        4770
    }
}
