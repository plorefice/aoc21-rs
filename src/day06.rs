pub fn parse_input(input: &str) -> [u64; 9] {
    input
        .split(',')
        .map(str::parse::<usize>)
        .fold([0; 9], |mut v, x| {
            v[x.unwrap()] += 1;
            v
        })
}

pub fn part_1(input: [u64; 9]) -> u64 {
    generate(input, 80)
}

pub fn part_2(input: [u64; 9]) -> u64 {
    generate(input, 256)
}

fn generate(mut input: [u64; 9], gens: usize) -> u64 {
    for _ in 0..gens {
        let spawn = input[0];
        input.rotate_left(1);
        input[6] += spawn;
    }
    input.iter().sum()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day06.txt"))),
        352195
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day06.txt"))),
        1600306001288
    }
}
