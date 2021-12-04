pub struct Board {
    entries: Vec<Vec<usize>>,
    marked: [[bool; 5]; 5],
}

impl Board {
    pub fn mark(&mut self, number: usize) -> bool {
        'out: for x in 0..5 {
            for y in 0..5 {
                if self.entries[x][y] == number {
                    self.marked[x][y] = true;
                    break 'out;
                }
            }
        }

        (0..5).any(|x| self.marked[x].iter().all(|e| *e))
            || (0..5).any(|y| self.marked.iter().all(|col| col[y]))
    }

    pub fn score(&self) -> usize {
        self.entries
            .iter()
            .flatten()
            .zip(self.marked.iter().flatten())
            .filter_map(|(n, marked)| if !marked { Some(n) } else { None })
            .sum()
    }
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut boards = input.split("\n\n");

    let numbers = boards
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()
        .unwrap();

    let boards = boards
        .map(|board| Board {
            entries: board
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(str::parse::<usize>)
                        .collect::<Result<_, _>>()
                        .unwrap()
                })
                .collect(),
            marked: Default::default(),
        })
        .collect();

    (numbers, boards)
}

pub fn part_1((numbers, mut boards): (Vec<usize>, Vec<Board>)) -> usize {
    for number in numbers {
        for board in &mut boards {
            if board.mark(number) {
                return number * board.score();
            }
        }
    }
    unreachable!()
}

pub fn part_2((numbers, mut boards): (Vec<usize>, Vec<Board>)) -> usize {
    for number in numbers {
        let mut i = 0;
        while i < boards.len() {
            if boards[i].mark(number) {
                let winner = boards.remove(i);
                if boards.is_empty() {
                    return number * winner.score();
                }
            } else {
                i += 1;
            }
        }
    }
    unreachable!()
}

crate::solutions! {
    p1 => {
        part_1(parse_input(include_str!("../inputs/day04.txt"))),
        34506
    },
    p2 => {
        part_2(parse_input(include_str!("../inputs/day04.txt"))),
        7686
    }
}
