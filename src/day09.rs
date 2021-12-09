use std::collections::VecDeque;

pub struct Floor {
    heightmap: Vec<u32>,
    width: usize,
}

impl Floor {
    pub fn higher_adjacent_positions(&self, i: usize) -> (Vec<usize>, bool) {
        let (x, y) = ((i % self.width) as isize, (i / self.width) as isize);

        let wi = self.width as isize;
        let hi = (self.heightmap.len() / self.width) as isize;

        let neighbours = [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)];
        let score = self.heightmap[i];

        let mut higher = Vec::with_capacity(4);
        let mut minimum = true;

        for (dx, dy) in neighbours {
            if dx < 0 || dx >= wi || dy < 0 || dy >= hi {
                continue;
            }
            let i = (dy * wi + dx) as usize;
            if score < self.heightmap[i] {
                higher.push(i);
            } else {
                minimum = false;
            }
        }

        (higher, minimum)
    }

    pub fn risk_at(&self, i: usize) -> u32 {
        if self.higher_adjacent_positions(i).1 {
            self.heightmap[i] + 1
        } else {
            0
        }
    }
}

pub fn parse_input(input: &str) -> Floor {
    let width = input.find('\n').unwrap();

    let heightmap = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    Floor { heightmap, width }
}

pub fn part_1(floor: &Floor) -> u32 {
    (0..floor.heightmap.len()).fold(0, |risk, i| risk + floor.risk_at(i))
}

pub fn part_2(floor: &Floor) -> usize {
    let mut visited = vec![false; floor.heightmap.len()];

    let mut baisins = (0..floor.heightmap.len()).fold(vec![], |mut baisins, i| {
        if let Some(size) = find_baisin(floor, i, &mut visited) {
            baisins.push(size);
        }
        baisins
    });

    baisins.sort_by(|a, b| b.cmp(a));
    baisins[0] * baisins[1] * baisins[2]
}

fn find_baisin(floor: &Floor, i: usize, visited: &mut [bool]) -> Option<usize> {
    // Start looking from low points only
    if floor.risk_at(i) == 0 {
        return None;
    }

    let mut candidates = VecDeque::from_iter([i]);
    let mut size = 0;

    while let Some(pos) = candidates.pop_front() {
        if visited[pos] {
            continue;
        }

        visited[pos] = true;
        size += 1;

        for i in floor.higher_adjacent_positions(pos).0.into_iter() {
            if floor.heightmap[i] != 9 {
                candidates.push_back(i);
            }
        }
    }

    Some(size)
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day09.txt"))),
        444
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day09.txt"))),
        1168440
    }
}
