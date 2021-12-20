use std::collections::HashSet;

pub fn parse_input(input: &str) -> (Vec<bool>, HashSet<(i32, i32)>) {
    let (template, input) = input.split_once("\n\n").unwrap();

    let template = template.bytes().map(|b| b == b'#').collect();

    let input = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, b)| {
                if b == b'#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    (template, input)
}

pub fn part_1(input: (Vec<bool>, HashSet<(i32, i32)>)) -> usize {
    solve(input, 2)
}

pub fn part_2(input: (Vec<bool>, HashSet<(i32, i32)>)) -> usize {
    solve(input, 50)
}

pub fn solve((template, mut image): (Vec<bool>, HashSet<(i32, i32)>), iterations: usize) -> usize {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = image.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(x_min, x_max, y_min, y_max), &(x, y)| {
            (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
        },
    );

    let mut outer_rim = 0;

    for _ in 0..iterations {
        let mut output =
            HashSet::with_capacity(((y_max - y_min + 3) * (x_max - x_min + 3)) as usize);

        for y in y_min - 1..=y_max + 1 {
            for x in x_min - 1..=x_max + 1 {
                let idx = adjacent((x, y)).iter().fold(0, |idx, &(x, y)| {
                    let lsb = if x < x_min || x > x_max || y < y_min || y > y_max {
                        outer_rim
                    } else {
                        image.contains(&(x, y)) as i32
                    };

                    (idx << 1) | lsb
                });

                if template[idx as usize] {
                    output.insert((x, y));
                }
            }
        }

        image = output;
        outer_rim ^= 1;

        x_min -= 1;
        x_max += 1;
        y_min -= 1;
        y_max += 1;
    }

    image.len()
}

fn adjacent((x, y): (i32, i32)) -> [(i32, i32); 9] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day20.txt"))),
        5081
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day20.txt"))),
        15088
    }
}
