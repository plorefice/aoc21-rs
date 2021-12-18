use std::{cell::RefCell, iter::Sum, ops::Add, rc::Rc};

use itertools::Itertools;

#[derive(Debug)]
pub enum Number {
    Regular(Rc<RefCell<u64>>),
    Pair(Box<Number>, Box<Number>),
}

#[derive(Debug)]
pub struct Explosion {
    target: (u64, u64),
    left: Option<Rc<RefCell<u64>>>,
    right: Option<Rc<RefCell<u64>>>,
}

impl Number {
    pub fn magnitude(&self) -> u64 {
        match self {
            Number::Regular(n) => *n.borrow(),
            Number::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn reduce(&mut self) {
        // Keep going until self either can be exploded (first) or split (second)
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        if let Some(explosion) = self.find_explosion(0) {
            if let Some(left) = explosion.left {
                *left.borrow_mut() += explosion.target.0;
            }
            if let Some(right) = explosion.right {
                *right.borrow_mut() += explosion.target.1;
            }
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Regular(x) if *x.borrow() >= 10 => {
                let x = *x.borrow();

                *self = Number::Pair(
                    Box::new(Number::Regular(Rc::new(RefCell::new(x / 2)))),
                    Box::new(Number::Regular(Rc::new(RefCell::new((x + 1) / 2)))),
                );
                true
            }
            Number::Regular(_) => false,
            Number::Pair(l, r) => l.split() || r.split(),
        }
    }

    fn find_explosion(&mut self, nest: usize) -> Option<Explosion> {
        match (&mut *self, nest) {
            (Number::Regular(_), _) => None,
            (Number::Pair(a, b), 4) => {
                let res = Some(Explosion {
                    target: match (&**a, &**b) {
                        (Number::Regular(a), Number::Regular(b)) => (*a.borrow(), *b.borrow()),
                        _ => unreachable!(),
                    },
                    left: None,
                    right: None,
                });

                *self = Number::Regular(Rc::default());
                res
            }
            (Number::Pair(sl, sr), _) => match sl.find_explosion(nest + 1) {
                Some(mut explosion) => {
                    if explosion.right.is_none() {
                        explosion.right = sr.find_regular_right();
                    }
                    Some(explosion)
                }
                None => {
                    if let Some(mut explosion) = sr.find_explosion(nest + 1) {
                        if explosion.left.is_none() {
                            explosion.left = sl.find_regular_left();
                        }
                        Some(explosion)
                    } else {
                        None
                    }
                }
            },
        }
    }

    fn find_regular_left(&self) -> Option<Rc<RefCell<u64>>> {
        match self {
            Number::Regular(l) => Some(l.clone()),
            Number::Pair(l, r) => r.find_regular_left().or_else(|| l.find_regular_left()),
        }
    }

    fn find_regular_right(&self) -> Option<Rc<RefCell<u64>>> {
        match self {
            Number::Regular(l) => Some(l.clone()),
            Number::Pair(l, r) => l.find_regular_right().or_else(|| r.find_regular_right()),
        }
    }

    pub fn deep_clone(&self) -> Number {
        match self {
            Number::Regular(r) => Number::Regular(Rc::new(RefCell::new(*r.borrow()))),
            Number::Pair(a, b) => Number::Pair(Box::new(a.deep_clone()), Box::new(b.deep_clone())),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Number::Pair(Box::new(self), Box::new(rhs));
        res.reduce();
        res
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
        .map(|line| parse::number(line).unwrap().1)
        .collect()
}

pub fn part_1(numbers: Vec<Number>) -> u64 {
    numbers.into_iter().sum::<Number>().magnitude()
}

pub fn part_2(numbers: Vec<Number>) -> u64 {
    numbers
        .iter()
        .permutations(2)
        .map(|nums| (nums[0].deep_clone() + nums[1].deep_clone()).magnitude())
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

mod parse {
    use std::{cell::RefCell, rc::Rc};

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        combinator::map_res,
        AsChar, IResult,
    };

    use crate::day18::Number;

    pub fn number(input: &str) -> IResult<&str, Number> {
        let (input, number) = alt((regular, pair))(input)?;
        Ok((input, number))
    }

    fn regular(input: &str) -> IResult<&str, Number> {
        map_res(take_while(|c: char| c.is_dec_digit()), |s: &str| {
            s.parse::<u64>()
                .map(|x| Number::Regular(Rc::new(RefCell::new(x))))
        })(input)
    }

    fn pair(input: &str) -> IResult<&str, Number> {
        let (input, _) = tag("[")(input)?;
        let (input, left) = alt((regular, pair))(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, right) = alt((regular, pair))(input)?;
        let (input, _) = tag("]")(input)?;

        Ok((input, Number::Pair(Box::new(left), Box::new(right))))
    }
}
