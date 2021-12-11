pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect()
}

pub fn part_1(mut input: Vec<u32>) -> usize {
    let mut total_flashes = 0;

    for _ in 0..100 {
        let mut flashed = vec![false; input.len()];

        for x in 0..10 {
            for y in 0..10 {
                total_flashes += step(&mut input, &mut flashed, (x, y));
            }
        }

        for (x, flashed) in input.iter_mut().zip(flashed.iter()) {
            if *flashed {
                *x = 0;
            }
        }
    }

    total_flashes
}

pub fn part_2(mut input: Vec<u32>) -> usize {
    for i in 1.. {
        let mut flashed = vec![false; input.len()];

        for x in 0..10 {
            for y in 0..10 {
                step(&mut input, &mut flashed, (x, y));
            }
        }

        if flashed.iter().all(|&f| f) {
            return i;
        }

        for (x, flashed) in input.iter_mut().zip(flashed.iter()) {
            if *flashed {
                *x = 0;
            }
        }
    }

    unreachable!()
}

fn step(input: &mut [u32], flashed: &mut [bool], (x, y): (i32, i32)) -> usize {
    let i = (y * 10 + x) as usize;
    let mut flashes = 0;

    input[i] += 1;
    if input[i] > 9 && !flashed[i] {
        flashed[i] = true;
        flashes += 1;

        let deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dx, dy) in deltas {
            let (xi, yi) = (x + dx, y + dy);
            if !(0..10).contains(&xi) || !(0..10).contains(&yi) {
                continue;
            }

            flashes += step(input, flashed, (xi, yi));
        }
    }

    flashes
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day11.txt"))),
        1675
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day11.txt"))),
        515
    }
}
