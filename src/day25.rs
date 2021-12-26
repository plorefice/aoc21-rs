use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    East,
    South,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    cucumbers: Vec<Vec<Option<Direction>>>,
    size: (usize, usize),
}

pub fn parse_input(input: &str) -> Grid {
    let lines = input.lines().collect_vec();
    let size = (lines.len(), lines[0].len());

    let cucumbers = lines
        .iter()
        .map(|s| {
            s.bytes()
                .map(|b| match b {
                    b'.' => None,
                    b'>' => Some(Direction::East),
                    b'v' => Some(Direction::South),
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    Grid { cucumbers, size }
}

pub fn part_1(mut grid: Grid) -> usize {
    for iter in 1.. {
        let mut next = grid.clone();

        for row in 0..grid.size.0 {
            for col in 0..grid.size.1 {
                let idx = (col + 1) % grid.size.1;

                if grid.cucumbers[row][col] == Some(Direction::East)
                    && grid.cucumbers[row][idx] == None
                {
                    next.cucumbers[row][col] = None;
                    next.cucumbers[row][idx] = Some(Direction::East);
                }
            }
        }

        let updated = next.clone();

        for row in 0..updated.size.0 {
            for col in 0..updated.size.1 {
                let idx = (row + 1) % updated.size.0;

                if updated.cucumbers[row][col] == Some(Direction::South)
                    && updated.cucumbers[idx][col] == None
                {
                    next.cucumbers[row][col] = None;
                    next.cucumbers[idx][col] = Some(Direction::South);
                }
            }
        }

        if next == grid {
            return iter;
        }

        grid = next;
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day25.txt"))),
        278
    }
}
