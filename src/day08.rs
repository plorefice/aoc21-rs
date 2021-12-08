use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref SEGMENT_STATE_BY_DIGIT: [&'static [u8]; 10] = [
        &b"abcefg"[..],
        &b"cf"[..],
        &b"acdeg"[..],
        &b"acdfg"[..],
        &b"bcdf"[..],
        &b"abdfg"[..],
        &b"abdefg"[..],
        &b"acf"[..],
        &b"abcdefg"[..],
        &b"abcdfg"[..],
    ];
}

pub struct Input<'a> {
    patterns: Vec<&'a str>,
    output: Vec<&'a str>,
}

pub fn parse_input(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let (patterns, output) = line.split_once(" | ").unwrap();

            Input {
                patterns: patterns.split_ascii_whitespace().collect(),
                output: output.split_ascii_whitespace().collect(),
            }
        })
        .collect()
}

pub fn part_1(inputs: &[Input]) -> usize {
    inputs
        .iter()
        .map(|input| {
            input
                .output
                .iter()
                .filter(|out| [2, 4, 3, 7].contains(&out.len()))
                .count()
        })
        .sum()
}

pub fn part_2(inputs: &[Input]) -> usize {
    inputs.iter().map(decode).sum()
}

fn decode(input: &Input) -> usize {
    'next: for shuffle in b"abcdefg".iter().cloned().permutations(7) {
        let translate = |s: &str| -> Vec<u8> {
            s.bytes()
                .map(|b| shuffle[(b - b'a') as usize])
                .sorted()
                .collect::<Vec<_>>()
        };

        for pattern in &input.patterns {
            if !SEGMENT_STATE_BY_DIGIT.contains(&translate(pattern).as_slice()) {
                continue 'next;
            }
        }

        return input.output.iter().fold(0, |digits, out| {
            let translated = translate(out);

            let n = SEGMENT_STATE_BY_DIGIT
                .iter()
                .position(|v| v == &translated)
                .unwrap();

            digits * 10 + n
        });
    }

    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(&parse_input(include_str!("../inputs/day08.txt"))),
        301
    },
    p2 => {
        part_2(&parse_input(include_str!("../inputs/day08.txt"))),
        908067
    }
}
