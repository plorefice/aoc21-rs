use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &[&str]) -> u32 {
    input.iter().fold(0, |points, &line| match compile(line) {
        Err(ch) => points + illegal_score(ch),
        Ok(_) => points,
    })
}

pub fn part_2(input: &[&str]) -> u64 {
    let points = input
        .iter()
        .filter_map(|line| match compile(line) {
            Ok(score) => Some(score),
            Err(_) => None,
        })
        .sorted()
        .collect_vec();

    points[points.len() / 2]
}

fn compile(line: &str) -> Result<u64, char> {
    let mut stack = Vec::with_capacity(line.len() / 2);

    for ch in line.chars() {
        match (stack.last(), ch) {
            (_, '(') | (_, '[') | (_, '{') | (_, '<') => stack.push(ch),
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                stack.pop();
            }
            _ => return Err(ch),
        }
    }

    Ok(stack
        .iter()
        .rev()
        .fold(0, |points, &ch| points * 5 + context_score(ch)))
}

const fn illegal_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

const fn context_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day10.txt"))),
        362271
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day10.txt"))),
        1698395182
    }
}
