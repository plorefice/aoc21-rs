pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn part_1(input: &[usize]) -> usize {
    input.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

pub fn part_2(input: &[usize]) -> usize {
    part_1(
        &input
            .windows(3)
            .map(|tuple| tuple.iter().sum::<usize>())
            .collect::<Vec<_>>(),
    )
}

crate::solutions!(
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day01.txt"))),
        1529
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day01.txt"))),
        1567
    }
);
