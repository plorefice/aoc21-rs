pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(str::parse::<i32>)
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn part_1(input: &[i32]) -> u32 {
    solve(input, |x, pivot| (x - pivot).abs())
}

pub fn part_2(input: &[i32]) -> u32 {
    solve(input, |x, pivot| {
        let n = (x - pivot).abs();
        n * (n + 1) / 2
    })
}

fn solve<F>(input: &[i32], mut f: F) -> u32
where
    F: FnMut(&i32, i32) -> i32,
{
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut best = u32::MAX;
    for pos in min..=max {
        let score = input.iter().map(|p| f(p, pos)).sum::<i32>() as u32;
        best = u32::min(score, best);
    }

    best
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day07.txt"))),
        352331
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day07.txt"))),
        99266250
    }
}
