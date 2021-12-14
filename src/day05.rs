use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(isize, isize);

impl Point {
    pub fn line_to(&self, other: &Point) -> Vec<Point> {
        let dx = (other.0 - self.0).signum();
        let dy = (other.1 - self.1).signum();

        let mut pts =
            Vec::with_capacity(((other.0 - self.0).abs() + (other.1 - self.1).abs() + 1) as usize);

        let Point(mut x, mut y) = self;
        while x != other.0 + dx || y != other.1 + dy {
            pts.push(Point(x, y));
            x += dx;
            y += dy;
        }
        pts
    }
}

pub fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<_> = line
                .split(" -> ")
                .flat_map(|coord| coord.split(',').map(str::parse::<isize>))
                .collect::<Result<_, _>>()
                .unwrap();

            (Point(coords[0], coords[1]), Point(coords[2], coords[3]))
        })
        .collect()
}

pub fn part_1(input: &[(Point, Point)]) -> usize {
    let mut map = HashMap::<_, i32>::new();

    for (a, b) in input.iter().filter(|(a, b)| a.0 == b.0 || a.1 == b.1) {
        for pt in a.line_to(b) {
            *map.entry(pt).or_default() += 1;
        }
    }

    map.values().filter(|&&n| n > 1).count()
}

pub fn part_2(input: &[(Point, Point)]) -> usize {
    let mut map = HashMap::<_, i32>::new();

    for (a, b) in input {
        for pt in a.line_to(b) {
            *map.entry(pt).or_default() += 1;
        }
    }

    map.values().filter(|&&n| n > 1).count()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day05.txt"))),
        4728
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day05.txt"))),
        17717
    }
}
