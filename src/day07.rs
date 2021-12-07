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
    let mut a = *input.iter().min().unwrap();
    let mut b = *input.iter().max().unwrap();

    // Bisection over the position range (the score is a parabola)
    loop {
        let pivot = (a + b) / 2;

        // Memoization doesn't help here, too few steps for it to actually matter
        let mid = input.iter().map(|p| f(p, pivot)).sum::<i32>() as u32;
        let left = input.iter().map(|p| f(p, pivot - 1)).sum::<i32>() as u32;
        let right = input.iter().map(|p| f(p, pivot + 1)).sum::<i32>() as u32;

        // Local optimum == global optimum in a parabola
        if left > mid && right > mid {
            return mid;
        }

        // Update bounds
        if left < mid {
            b = pivot;
        } else {
            a = pivot;
        }
    }
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
