use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Range(i64, i64);

impl Range {
    pub fn length(&self) -> i64 {
        self.1 - self.0 + 1
    }

    pub fn contains(&self, i: i64) -> bool {
        self.0 <= i && i <= self.1
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }

    pub fn split(&self, other: &Self) -> Vec<Range> {
        if other.0 <= self.0 {
            if other.1 >= self.1 {
                vec![*self]
            } else {
                vec![Range(self.0, other.1), Range(other.1 + 1, self.1)]
            }
        } else if other.1 >= self.1 {
            vec![Range(self.0, other.0 - 1), Range(other.0, self.1)]
        } else {
            vec![
                Range(self.0, other.0 - 1),
                Range(other.0, other.1),
                Range(other.1 + 1, self.1),
            ]
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    pub fn volume(&self) -> i64 {
        self.x.length() * self.y.length() * self.z.length()
    }

    pub fn contains(&self, x: i64, y: i64, z: i64) -> bool {
        self.x.contains(x) && self.y.contains(y) && self.z.contains(z)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y) && self.z.intersects(&other.z)
    }

    pub fn subtract(&self, other: &Self) -> Vec<Cuboid> {
        if !self.intersects(other) {
            return vec![*self];
        }

        let xs = self.x.split(&other.x);
        let ys = self.y.split(&other.y);
        let zs = self.z.split(&other.z);

        let mut res = vec![];

        for &x in &xs {
            for &y in &ys {
                for &z in &zs {
                    if self.contains(x.0, y.0, z.0) && !other.contains(x.0, y.0, z.0) {
                        res.push(Cuboid { x, y, z });
                    }
                }
            }
        }

        res
    }
}

pub type Instruction = (bool, Cuboid);

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (command, coords) = l.split_once(' ').unwrap();
            let (x, y, z) = coords
                .split(',')
                .map(|c| {
                    let (low, high) = c.split_once("..").unwrap();
                    Range(low[2..].parse().unwrap(), high.parse().unwrap())
                })
                .collect_tuple()
                .unwrap();

            (command == "on", Cuboid { x, y, z })
        })
        .collect()
}

pub fn part_1(instructions: Vec<Instruction>) -> i64 {
    let mut cuboids = Vec::<Cuboid>::new();

    for (on, cuboid) in instructions.into_iter().take_while(|(_, c)| {
        c.x.0 >= -50 && c.x.1 <= 50 && c.y.0 >= -50 && c.y.1 <= 50 && c.z.0 >= -50 && c.z.1 <= 50
    }) {
        cuboids = cuboids
            .iter()
            .map(|c| c.subtract(&cuboid))
            .flatten()
            .collect();

        if on {
            cuboids.push(cuboid);
        }

        dbg!(&cuboids);
    }

    cuboids.iter().map(Cuboid::volume).sum()
}

pub fn part_2(instructions: Vec<Instruction>) -> i64 {
    let mut cuboids = Vec::<Cuboid>::new();

    for (on, cuboid) in instructions {
        cuboids = cuboids
            .iter()
            .map(|c| c.subtract(&cuboid))
            .flatten()
            .collect();

        if on {
            cuboids.push(cuboid);
        }
    }

    cuboids.iter().map(Cuboid::volume).sum()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day22.txt"))),
        570915
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day22.txt"))),
        1268313839428137
    }
}
